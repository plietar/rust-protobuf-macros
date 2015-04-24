use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;

enum Accessor {
    Get,
    Mut,
    Set
}

fn field_accessor(cx: &ExtCtxt, ident: &ast::Ident, mode: Accessor) -> ast::Ident {
    let prefix = match mode {
        Accessor::Get => "get_",
        Accessor::Mut => "mut_",
        Accessor::Set => "set_"
    };

    cx.ident_of(&(prefix.to_string() + ident.as_str()))
}

pub fn field_get(cx: &ExtCtxt, sp: Span, parent: P<ast::Expr>, key: &[ast::Ident], mutable: bool) -> P<ast::Expr> {
    assert!(key.len() > 0);

    let mut e = parent;
    for ident in key {
        let accessor = if mutable {
            field_accessor(cx, ident, Accessor::Mut)
        } else {
            field_accessor(cx, ident, Accessor::Get)
        };

        e = cx.expr_method_call(
            sp,
            e,
            accessor,
            Vec::new())
    }

    return e;
}

pub fn field_set(cx: &ExtCtxt, sp: Span, parent: P<ast::Expr>, key: &[ast::Ident], value: P<ast::Expr>) -> P<ast::Expr> {
    assert!(key.len() > 0);

    let parent = if key.len() > 1 {
        field_get(cx, sp, parent, key.init(), true)
    } else {
        parent
    };

    let f = field_accessor(cx, key.last().unwrap(), Accessor::Set);
    cx.expr_method_call(
        sp,
        parent,
        f,
        vec![value])
}

