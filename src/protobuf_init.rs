use aster::expr::ExprBuilder;
use std::clone::Clone;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager, DummyResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::PResult;
use syntax::parse::parser::Parser;
use syntax::ptr::P;

use util;
use util::{Value, Field, Message, RHSParser};

struct ExprParser;
impl RHSParser for ExprParser {
    type RHS = P<ast::Expr>;
    fn parse<'a>(&mut self, parser: &mut Parser<'a>) -> PResult<'a, Self::RHS> {
        parser.parse_expr()
    }
}

fn parse_protobuf<'a>(cx: &mut ExtCtxt<'a>, tts: &[ast::TokenTree]) -> PResult<'a, (P<ast::Expr>, Message<P<ast::Expr>>)> {
    let mut parser = cx.new_parser_from_tts(&tts.to_vec());
    util::MacroParser::new(&mut parser, ExprParser).parse_macro()
}

fn convert_single_value(cx: &mut ExtCtxt, value: P<ast::Expr>) -> P<ast::Expr> {
    let use_into = if let ast::ExprKind::Lit(ref lit) = value.node {
        if let ast::LitKind::Str(..) = lit.node {
            true
        } else {
            false
        }
    } else {
        false
    };

    if use_into {
        ExprBuilder::new()
                    .span(value.span)
                    .call().span(value.span).path().global().ids(&["std", "convert", "Into", "into"]).build()
                    .arg().span(value.span).build(value)
                    .build()
    } else {
        value
    }
}


fn emit_repeated(cx: &mut ExtCtxt, sp: Span, value: Value<P<ast::Expr>>, parent: P<ast::Expr>) -> ast::Stmt {
    let e = match value {
        Value::SingleValue(expr) => {
            let f_push = cx.ident_of("push");
            let expr = convert_single_value(cx, expr);

            cx.expr_method_call(
                sp,
                parent,
                f_push,
                vec![expr])
        },
        Value::MessageValue(msg) => {
            let f_push_default = cx.ident_of("push_default");
            let e_push_default = cx.expr_method_call(
                sp,
                parent,
                f_push_default,
                Vec::new());

            emit_message(cx, sp, msg, e_push_default)
        },
        Value::RepeatedValue(_) => panic!("Cannot nest repeated fields")
    };

    cx.stmt_expr(e)
}

fn emit_field(cx: &mut ExtCtxt, sp: Span, field: Field<P<ast::Expr>>, parent: P<ast::Expr>) -> P<ast::Expr> {
    let Field(key, value) = field;

    match value {
        Value::SingleValue(expr) => {
            let expr = convert_single_value(cx, expr);
            util::field_set(cx, parent, &key, expr)
        },
        Value::MessageValue(msg) => {
            let e_msg = util::field_get(cx, parent, &key, true);
            emit_message(cx, sp, msg, e_msg)
        },
        Value::RepeatedValue(values) => {
            let mut stmts = Vec::new();

            let i_repeated = cx.ident_of("repeated");
            let e_repeated = cx.expr_ident(sp, i_repeated);

            let e_mut_xxx = util::field_get(cx, parent, &key, true);
            stmts.push(cx.stmt_let(sp, true, i_repeated, e_mut_xxx));

            for v in values {
                stmts.push(emit_repeated(cx, sp, v, e_repeated.clone()))
            }
            let block = cx.block(sp, stmts, None);
            cx.expr_block(block)
        },
    }
}

fn emit_message(cx: &mut ExtCtxt, sp: Span, msg: Message<P<ast::Expr>>, expr: P<ast::Expr>) -> P<ast::Expr>{
    let Message(fields) = msg;
    if fields.len() > 0 {
        let i_msg = cx.ident_of("msg");
        let e_msg = cx.expr_ident(sp, i_msg);

        let mut stmts = Vec::new();
        stmts.push(cx.stmt_let(sp, true, i_msg, expr));

        for f in fields {
            let e_field = emit_field(cx, sp, f, e_msg.clone());
            stmts.push(cx.stmt_expr(e_field));
        }

        let block = cx.block(sp, stmts, Some(e_msg));
        cx.expr_block(block)
    } else {
        expr
    }
}

pub fn macro_protobuf_init(cx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree]) -> Box<MacResult+'static> {
    match parse_protobuf(cx, tts) {
        Ok((expr, msg)) => {
            MacEager::expr(
                emit_message(cx, sp, msg, expr)
            )
        }
        Err(_) => {
           DummyResult::any(sp)
        }
    }
}
