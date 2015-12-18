use syntax::ast;
use syntax::codemap::{Spanned, Span, respan, spanned};
use syntax::ptr::P;
use syntax::parse::parser::Parser;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::parse::{token,PResult};

enum Accessor {
    Get,
    Mut,
    Set
}

pub trait ParserExt {
    fn parse_spanned_ident(&mut self) -> PResult<Spanned<ast::Ident>>;
}

impl <'a> ParserExt for Parser<'a> {
    fn parse_spanned_ident(&mut self) -> PResult<Spanned<ast::Ident>> {
        let lo = self.span.lo;
        let ident = try!(self.parse_ident());
        let hi = self.span.hi;
        
        Ok(spanned(lo, hi, ident))
    }
}

pub trait AstBuilderExt {
    fn stmt_let_pat(&self, sp: Span, pat: P<ast::Pat>, ex: P<ast::Expr>) -> P<ast::Stmt>;
}

impl <T : AstBuilder> AstBuilderExt for T {
    fn stmt_let_pat(&self, sp: Span, pat: P<ast::Pat>, ex: P<ast::Expr>) -> P<ast::Stmt> {
        let local = P(ast::Local {
            pat: pat,
            ty: None,
            init: Some(ex),
            id: ast::DUMMY_NODE_ID,
            span: sp,
            attrs: None,
        });

        let decl = respan(sp, ast::DeclLocal(local));
        P(respan(sp, ast::StmtDecl(P(decl), ast::DUMMY_NODE_ID)))
    }
}

fn field_accessor(cx: &ExtCtxt, ident: &Spanned<ast::Ident>, mode: Accessor) -> Spanned<ast::Ident> {
    let prefix = match mode {
        Accessor::Get => "get_",
        Accessor::Mut => "mut_",
        Accessor::Set => "set_"
    };

    respan(ident.span, cx.ident_of(&(prefix.to_string() + &ident.node.to_string())))
}

pub fn field_get(cx: &ExtCtxt, parent: P<ast::Expr>, key: &[Spanned<ast::Ident>], mutable: bool) -> P<ast::Expr> {
    assert!(key.len() > 0);

    let mut e = parent;
    for ident in key {
        let accessor = if mutable {
            field_accessor(cx, ident, Accessor::Mut)
        } else {
            field_accessor(cx, ident, Accessor::Get)
        };

        e = cx.expr_method_call(
            accessor.span,
            e,
            accessor.node,
            Vec::new())
    }

    return e;
}

pub fn field_set(cx: &ExtCtxt, parent: P<ast::Expr>, key: &[Spanned<ast::Ident>], value: P<ast::Expr>) -> P<ast::Expr> {
    let (ident, parent) = match key.split_last() {
        None => panic!("At least one ident is required"),
        Some((ident, [])) => (ident, parent),
        Some((ident, path)) => (ident, field_get(cx, parent, path, true))
    };
    let accessor = field_accessor(cx, ident, Accessor::Set);

    cx.expr_method_call(
        accessor.span,
        parent,
        accessor.node,
        vec![value])
}

pub trait RHSParser {
    type RHS;
    fn parse(&mut self, parser: &mut Parser) -> PResult<Self::RHS>;
}

pub struct MacroParser<'a, 'b: 'a, T : RHSParser> {
    parser: &'a mut Parser<'b>,
    rhs_parser: T
}

#[derive(Debug)]
pub enum Value<T> {
    SingleValue(T),
    MessageValue(Message<T>),
    RepeatedValue(Vec<Value<T>>),
}
#[derive(Debug)]
pub struct Field<T>(pub Vec<Spanned<ast::Ident>>, pub Value<T>);

#[derive(Debug)]
pub struct Message<T>(pub Vec<Field<T>>);

impl <'a,'b, T : RHSParser> MacroParser<'a,'b, T> {
    pub fn new(parser: &'a mut Parser<'b>, rhs_parser: T) -> MacroParser<'a, 'b, T> {
        MacroParser {
            parser: parser,
            rhs_parser: rhs_parser
        }
    }

    pub fn parse_macro(&mut self) -> PResult<(P<ast::Expr>, Message<T::RHS>)> {
        let expr = try!(self.parser.parse_expr());
        try!(self.parser.expect(&token::Comma));
        let msg = try!(self.parse_message());
        try!(self.parser.expect(&token::Eof));

        return Ok((expr, msg))
    }

    fn parse_message(&mut self) -> PResult<Message<T::RHS>> {
        try!(self.parser.expect(&token::OpenDelim(token::Brace)));

        let mut fields = Vec::new();

        while self.parser.token != token::CloseDelim(token::Brace) {
            let f = try!(self.parse_field());
            fields.push(f);

            try!(self.parser.expect_one_of(
                &[token::Comma],
                &[token::CloseDelim(token::Brace)]));
        }

        try!(self.parser.expect(&token::CloseDelim(token::Brace)));
        Ok(Message(fields))
    }

    fn parse_field(&mut self) -> PResult<Field<T::RHS>> {
        let ident = try!(self.parse_spanned_idents());

        match self.parser.token {
            token::Colon => {
                try!(self.parser.bump());
                Ok(Field(ident, try!(self.parse_value())))
            },
            token::FatArrow => {
                try!(self.parser.bump());
                Ok(Field(ident, try!(self.parse_compound())))
            },
            _ => Err(self.parser.unexpected())
        }
    }

    fn parse_spanned_idents(&mut self) -> PResult<Vec<Spanned<ast::Ident>>> {
        let mut vec = Vec::new();

        vec.push(try!(self.parser.parse_spanned_ident()));

        while try!(self.parser.eat(&token::Dot)) {
            vec.push(try!(self.parser.parse_spanned_ident()));
        }

        Ok(vec)
    }


    fn parse_compound(&mut self) -> PResult<Value<T::RHS>> {
        match self.parser.token {
            token::OpenDelim(token::Brace) => {
                Ok(Value::MessageValue(try!(self.parse_message())))
            },
            token::OpenDelim(token::Bracket) => {
                Ok(Value::RepeatedValue(try!(self.parse_repeated())))
            },
            _ => Err(self.parser.unexpected())
        }
    }

    fn parse_repeated(&mut self) -> PResult<Vec<Value<T::RHS>>> {
        try!(self.parser.expect(&token::OpenDelim(token::Bracket)));
        let mut values = Vec::new();

        while self.parser.token != token::CloseDelim(token::Bracket) {
            values.push(try!(self.parse_value()));

            try!(self.parser.expect_one_of(
                &[token::Comma],
                &[token::CloseDelim(token::Bracket)]));
        }

        try!(self.parser.expect(&token::CloseDelim(token::Bracket)));

        Ok(values)
    }

    fn parse_value(&mut self) -> PResult<Value<T::RHS>> {
        match self.parser.token {
            token::At => {
                try!(self.parser.bump());
                Ok(try!(self.parse_compound()))
            }
            _ => Ok(Value::SingleValue(try!(self.rhs_parser.parse(self.parser)))),
        }
    }

}


