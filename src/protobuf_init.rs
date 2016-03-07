use aster::expr::ExprBuilder;
use aster::stmt::StmtBuilder;

use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager, DummyResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::PResult;
use syntax::parse::parser::Parser;
use syntax::ptr::P;
use syntax::parse::token::gensym_ident;

use util;
use parser::{Value, Field, Message, MacroParser, RHSParser};

struct ExprParser;
impl RHSParser for ExprParser {
    type RHS = P<ast::Expr>;
    fn parse<'a>(&mut self, parser: &mut Parser<'a>) -> PResult<'a, Self::RHS> {
        parser.parse_expr()
    }
}

fn parse_protobuf<'a>(cx: &mut ExtCtxt<'a>,
                      tts: &[ast::TokenTree]) -> PResult<'a, (P<ast::Expr>, Message<P<ast::Expr>>)> {

    let mut parser = cx.new_parser_from_tts(&tts.to_vec());
    MacroParser::new(&mut parser, ExprParser).parse_macro()
}

fn convert_single_value(value: P<ast::Expr>) -> P<ast::Expr> {
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


fn emit_repeated_field(sp: Span,
                       parent: P<ast::Expr>,
                       value: Value<P<ast::Expr>>) -> ast::Stmt {

    match value {
        Value::SingleValue(expr) => {
            let expr = convert_single_value(expr);
            StmtBuilder::new().expr()
                        .method_call("push").build(parent)
                        .arg().build(expr)
                        .build()
        },
        Value::MessageValue(msg) => {
            let msg_expr = ExprBuilder::new()
                                       .method_call("push_default").build(parent)
                                       .build();

            StmtBuilder::new().semi().build(emit_message(sp, msg_expr, msg))
        },
        Value::RepeatedValue(_) => panic!("Cannot nest repeated fields")
    }
}

fn emit_field(sp: Span,
              parent: P<ast::Expr>,
              Field(key, value): Field<P<ast::Expr>>) -> ast::Stmt {

    match value {
        Value::SingleValue(expr) => {
            let expr = convert_single_value(expr);
            util::field_set(parent, &key, expr)
        },
        Value::MessageValue(msg) => {
            let msg_expr = util::field_get(parent, &key, true);
            StmtBuilder::new().semi().build(emit_message(sp, msg_expr, msg))
        },
        Value::RepeatedValue(values) => {
            let mut builder = StmtBuilder::new().semi().block();

            if values.len() > 0 {
                let i_repeated = gensym_ident("repeated");
                let e_repeated = ExprBuilder::new().id(i_repeated);

                builder = builder.stmt()
                                 .let_id(i_repeated)
                                 .build(util::field_get(parent, &key, true));

                for v in values {
                    let stmt = emit_repeated_field(sp, e_repeated.clone(), v);
                    builder = builder.with_stmt(stmt);
                }
            }

            builder.build()
        },
    }
}

fn emit_message(sp: Span,
                expr: P<ast::Expr>,
                Message(fields): Message<P<ast::Expr>>) -> P<ast::Expr> {

    if fields.len() > 0 {
        let mut builder = ExprBuilder::new().block();

        let i_msg = gensym_ident("msg");
        let e_msg = ExprBuilder::new().id(i_msg);

        builder = builder.stmt().let_().mut_id(i_msg)
                                .expr().build(expr);

        for field in fields {
            let stmt = emit_field(sp, e_msg.clone(), field);
            builder = builder.with_stmt(stmt);
        }

        builder.expr().build(e_msg)
    } else {
        expr
    }
}

pub fn macro_protobuf_init<'a>(cx: &mut ExtCtxt,
                               sp: Span,
                               tts: &[ast::TokenTree]) -> Box<MacResult+'a> {
    match parse_protobuf(cx, tts) {
        Ok((expr, msg)) => {
            MacEager::expr(
                emit_message(sp, expr, msg)
            )
        }
        Err(_) => {
           DummyResult::any(sp)
        }
    }
}
