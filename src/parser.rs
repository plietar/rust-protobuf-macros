use syntax::ast;
use syntax::codemap::{Spanned, spanned};
use syntax::ptr::P;
use syntax::parse::parser::Parser;
use syntax::parse::{token,PResult};

pub trait ParserExt<'a> {
    fn parse_spanned_ident(&mut self) -> PResult<'a, Spanned<ast::Ident>>;
}

impl <'a> ParserExt<'a> for Parser<'a> {
    fn parse_spanned_ident(&mut self) -> PResult<'a, Spanned<ast::Ident>> {
        let lo = self.span.lo;
        let ident = try!(self.parse_ident());
        let hi = self.span.hi;
        
        Ok(spanned(lo, hi, ident))
    }
}

pub trait RHSParser {
    type RHS;
    fn parse<'a>(&mut self, parser: &mut Parser<'a>) -> PResult<'a, Self::RHS>;
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

    pub fn parse_macro(&mut self) -> PResult<'b, (P<ast::Expr>, Message<T::RHS>)> {
        let expr = try!(self.parser.parse_expr());
        try!(self.parser.expect(&token::Comma));
        let msg = try!(self.parse_message());
        try!(self.parser.expect(&token::Eof));

        return Ok((expr, msg))
    }

    fn parse_message(&mut self) -> PResult<'b, Message<T::RHS>> {
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

    fn parse_field(&mut self) -> PResult<'b, Field<T::RHS>> {
        let ident = try!(self.parse_spanned_idents());

        match self.parser.token {
            token::Colon => {
                self.parser.bump();
                Ok(Field(ident, try!(self.parse_value())))
            },
            token::FatArrow => {
                self.parser.bump();
                Ok(Field(ident, try!(self.parse_compound())))
            },
            _ => self.parser.unexpected()
        }
    }

    fn parse_spanned_idents(&mut self) -> PResult<'b, Vec<Spanned<ast::Ident>>> {
        let mut vec = Vec::new();

        vec.push(try!(self.parser.parse_spanned_ident()));

        while self.parser.eat(&token::Dot) {
            vec.push(try!(self.parser.parse_spanned_ident()));
        }

        Ok(vec)
    }


    fn parse_compound(&mut self) -> PResult<'b, Value<T::RHS>> {
        match self.parser.token {
            token::OpenDelim(token::Brace) => {
                Ok(Value::MessageValue(try!(self.parse_message())))
            },
            token::OpenDelim(token::Bracket) => {
                Ok(Value::RepeatedValue(try!(self.parse_repeated())))
            },
            _ => self.parser.unexpected()
        }
    }

    fn parse_repeated(&mut self) -> PResult<'b, Vec<Value<T::RHS>>> {
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

    fn parse_value(&mut self) -> PResult<'b, Value<T::RHS>> {
        match self.parser.token {
            token::At => {
                self.parser.bump();
                Ok(try!(self.parse_compound()))
            }
            _ => Ok(Value::SingleValue(try!(self.rhs_parser.parse(self.parser)))),
        }
    }

}
