use aster::stmt::StmtBuilder;
use aster::expr::ExprBuilder;
use aster::ident::ToIdent;

use syntax::ast;
use syntax::codemap::{Spanned, Span, respan};
use syntax::ptr::P;
use syntax::ext::build::AstBuilder;

enum Accessor {
    Get,
    Mut,
    Set
}

pub trait AstBuilderExt {
    fn stmt_let_pat(&self, sp: Span, pat: P<ast::Pat>, ex: P<ast::Expr>) -> ast::Stmt;
}

impl <T : AstBuilder> AstBuilderExt for T {
    fn stmt_let_pat(&self, sp: Span, pat: P<ast::Pat>, ex: P<ast::Expr>) -> ast::Stmt {
        let local = P(ast::Local {
            pat: pat,
            ty: None,
            init: Some(ex),
            id: ast::DUMMY_NODE_ID,
            span: sp,
            attrs: None,
        });

        let decl = respan(sp, ast::DeclKind::Local(local));
        respan(sp, ast::StmtKind::Decl(P(decl), ast::DUMMY_NODE_ID))
    }
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
        Some((ident, [])) => (ident, parent),
        Some((ident, path)) => (ident, field_get(parent, path, true))
    };

    let accessor = field_accessor(ident, Accessor::Set);

    StmtBuilder::new().expr()
                .method_call(accessor.node).build(parent)
                .arg().build(value)
                .build()
}
