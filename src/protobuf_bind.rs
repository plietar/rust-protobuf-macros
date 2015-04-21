use std::clone::Clone;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager, DummyResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::{token,PResult};
use syntax::parse::parser::Parser;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;
use syntax::codemap::respan;

#[derive(Debug)]
enum Value {
    SingleValue(ast::Ident),
    MessageValue(Message),
}
#[derive(Debug)]
struct Field(ast::Ident, Value);
#[derive(Debug)]
struct Message(Vec<Field>);

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

fn parse_compound(parser: &mut Parser) -> PResult<Value> {
    match parser.token {
        token::OpenDelim(token::Brace) => {
            Ok(Value::MessageValue(try!(parse_message(parser))))
        },
        //token::OpenDelim(token::Bracket) => {
        //    Ok(Value::RepeatedValue(try!(parse_repeated(parser))))
        //},
        _ => Err(parser.unexpected())
    }
}

fn parse_value(parser: &mut Parser) -> PResult<Value> {
    match parser.token {
        token::At => {
            try!(parser.bump());
            Ok(try!(parse_compound(parser)))
        }
        _ => Ok(Value::SingleValue(try!(parser.parse_ident()))),
    }
}

fn parse_field(parser: &mut Parser) -> PResult<Field> {
    let ident = try!(parser.parse_ident());

    match parser.token {
        token::Colon => {
            try!(parser.bump());
            Ok(Field(ident, try!(parse_value(parser))))
        },
        token::FatArrow => {
            try!(parser.bump());
            Ok(Field(ident, try!(parse_compound(parser))))
        },
        _ => Err(parser.unexpected())
    }
}

fn parse_message(parser: &mut Parser) -> PResult<Message> {
    try!(parser.expect(&token::OpenDelim(token::Brace)));

    let mut fields = Vec::new();

    while parser.token != token::CloseDelim(token::Brace) {
        let f = try!(parse_field(parser));
        fields.push(f);

        try!(parser.expect_one_of(
            &[token::Comma],
            &[token::CloseDelim(token::Brace)]));
    }

    try!(parser.expect(&token::CloseDelim(token::Brace)));
    Ok(Message(fields))
}

fn parse_protobuf(cx: &mut ExtCtxt, tts: &[ast::TokenTree]) -> PResult<(P<ast::Expr>, Message)> {
    let mut parser = cx.new_parser_from_tts(&tts.to_vec());

    let expr = try!(parser.parse_expr_nopanic());
    try!(parser.expect(&token::Comma));
    let msg = try!(parse_message(&mut parser));
    try!(parser.expect(&token::Eof));

    return Ok((expr, msg))
}

fn emit_field(cx: &mut ExtCtxt, sp: Span, field: Field, parent: P<ast::Expr>) -> (P<ast::Pat>, P<ast::Expr>) {
    let Field(key, value) = field;

    match value {
        Value::SingleValue(ident) => {
            let pat = cx.pat_ident(sp, ident);

            let f_get_xxx = cx.ident_of(&format!("get_{}", key));
            let e = cx.expr_method_call(
                sp,
                parent,
                f_get_xxx,
                Vec::new()
            );

            (pat, e)
        },
        Value::MessageValue(msg) => {
            let i_msg = cx.ident_of("msg");
            let e_msg = cx.expr_ident(sp, i_msg);

            let f_get_xxx = cx.ident_of(&format!("get_{}", key));
            let e = cx.expr_method_call(
                sp,
                parent,
                f_get_xxx,
                Vec::new()
            );

            let stmts = vec![
                cx.stmt_let(sp, false, i_msg, e)
            ];
            let (pat, value) = emit_message(cx, sp, msg, e_msg);

            let block = cx.block(sp, stmts, Some(value));

            (pat, cx.expr_block(block))
        }
    }
}

fn emit_message(cx: &mut ExtCtxt, sp: Span, msg: Message, expr: P<ast::Expr>) -> (P<ast::Pat>, P<ast::Expr>) {
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


