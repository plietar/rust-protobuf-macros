use std::clone::Clone;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager, DummyResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::PResult;
use syntax::parse::parser::Parser;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;
use syntax::codemap::respan;

use util;
use util::{Value, Field, Message, RHSParser};

fn stmt_let_pat(sp: Span, pat: P<ast::Pat>, ex: P<ast::Expr>) -> P<ast::Stmt> {
    let local = P(ast::Local {
        pat: pat,
        ty: None,
        init: Some(ex),
        id: ast::DUMMY_NODE_ID,
        span: sp,
        source: ast::LocalLet,
    });

    let decl = respan(sp, ast::DeclLocal(local));
    P(respan(sp, ast::StmtDecl(P(decl), ast::DUMMY_NODE_ID)))
}

struct IdentParser;
impl RHSParser for IdentParser {
    type RHS = ast::Ident;
    fn parse(&mut self, parser: &mut Parser) -> PResult<Self::RHS> {
        parser.parse_ident()
    }
}

fn parse_protobuf(cx: &mut ExtCtxt, tts: &[ast::TokenTree]) -> PResult<(P<ast::Expr>, Message<ast::Ident>)> {
    let mut parser = cx.new_parser_from_tts(&tts.to_vec());
    util::MacroParser::new(&mut parser, IdentParser).parse_macro()
}

fn emit_field(cx: &mut ExtCtxt, sp: Span, field: Field<ast::Ident>, parent: P<ast::Expr>) -> (P<ast::Pat>, P<ast::Expr>) {
    let Field(key, value) = field;

    match value {
        Value::SingleValue(ident) => {
            let pat = cx.pat_ident(sp, ident);

            let e = util::field_get(cx, sp, parent, &key, false);

            (pat, e)
        },
        Value::MessageValue(msg) => {
            let i_msg = cx.ident_of("msg");
            let e_msg = cx.expr_ident(sp, i_msg);

            let e = util::field_get(cx, sp, parent, &key, false);

            let stmts = vec![
                cx.stmt_let(sp, false, i_msg, e)
            ];
            let (pat, value) = emit_message(cx, sp, msg, e_msg);

            let block = cx.block(sp, stmts, Some(value));

            (pat, cx.expr_block(block))
        },
        Value::RepeatedValue(_) => {
            panic!("protobuf_bind! does not support repeated fields");
        }
    }
}

fn emit_message(cx: &mut ExtCtxt, sp: Span, msg: Message<ast::Ident>, expr: P<ast::Expr>) -> (P<ast::Pat>, P<ast::Expr>) {
    let mut pats = Vec::new();
    let mut values = Vec::new();

    let Message(fields) = msg;
    for f in fields {
        let (pat, value) = emit_field(cx, sp, f, expr.clone());
        pats.push(pat);
        values.push(value);
    }

    (cx.pat_tuple(sp, pats), cx.expr_tuple(sp, values))
}

pub fn macro_protobuf_bind(cx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree]) -> Box<MacResult+'static> {
    match parse_protobuf(cx, tts) {
        Ok((expr, msg)) => {
            let (pat, value) = emit_message(cx, sp, msg, expr);
            MacEager::stmts(SmallVector::one(
                stmt_let_pat(sp, pat, value)
            ))
        }
        Err(_) => {
            DummyResult::any(sp)
        }
    }
}


