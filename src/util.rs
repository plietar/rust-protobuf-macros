use aster::stmt::StmtBuilder;
use aster::expr::ExprBuilder;
use aster::ident::ToIdent;

use syntax::ast;
use syntax::codemap::{Spanned, respan};
use syntax::ptr::P;

enum Accessor {
    Get,
    Mut,
    Set
}

fn field_accessor(ident: &Spanned<ast::Ident>, mode: Accessor) -> Spanned<ast::Ident> {
    let prefix = match mode {
        Accessor::Get => "get_",
        Accessor::Mut => "mut_",
        Accessor::Set => "set_"
    };

    respan(ident.span, ToIdent::to_ident(&(prefix.to_string() + &ident.node.to_string())))
}

pub fn field_get(parent: P<ast::Expr>,
                 key: &[Spanned<ast::Ident>],
                 mutable: bool) -> P<ast::Expr> {

    assert!(key.len() > 0);

    let mut e = parent;
    for ident in key {
        let accessor = if mutable {
            field_accessor(ident, Accessor::Mut)
        } else {
            field_accessor(ident, Accessor::Get)
        };

        e = ExprBuilder::new()
                        .method_call(accessor.node).build(e)
                        .build();
    }

    return e;
}

pub fn field_set(parent: P<ast::Expr>,
                 key: &[Spanned<ast::Ident>],
                 value: P<ast::Expr>) -> ast::Stmt {

    let (ident, parent) = match key.split_last() {
        None => panic!("At least one ident is required"),
        Some((ident, path)) => {
            if path.is_empty() {
                (ident, parent)
            } else {
                (ident, field_get(parent, path, true))
            }
        }
    };

    let accessor = field_accessor(ident, Accessor::Set);

    StmtBuilder::new().expr()
                .method_call(accessor.node).build(parent)
                .arg().build(value)
                .build()
}
