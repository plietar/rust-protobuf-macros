#![cfg_attr(not(feature = "with-syntex"), feature(plugin_registrar, rustc_private))]

extern crate aster;

#[cfg(feature = "with-syntex")] extern crate syntex;
#[cfg(feature = "with-syntex")] extern crate syntex_syntax as syntax;
#[cfg(feature = "with-syntex")] use syntex::Registry;

#[cfg(not(feature = "with-syntex"))] extern crate rustc;
#[cfg(not(feature = "with-syntex"))] extern crate syntax;
#[cfg(not(feature = "with-syntex"))] extern crate rustc_plugin;
#[cfg(not(feature = "with-syntex"))] use rustc_plugin::Registry;

mod util;
mod parser;
mod protobuf_init;
mod protobuf_bind;

#[cfg(not(feature = "with-syntex"))]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("protobuf_init", protobuf_init::macro_protobuf_init);
    reg.register_macro("protobuf_bind", protobuf_bind::macro_protobuf_bind);
}

#[cfg(feature = "with-syntex")]
pub fn expand<S, D>(src: S, dst: D) -> Result<(), syntex::Error>
    where S: AsRef<std::path::Path>,
          D: AsRef<std::path::Path>,
{
    let mut registry = Registry::new();
    registry.add_macro("protobuf_init", protobuf_init::macro_protobuf_init);
    registry.add_macro("protobuf_bind", protobuf_bind::macro_protobuf_bind);
    registry.expand("", src.as_ref(), dst.as_ref())
}
