#![feature(plugin_registrar, rustc_private)]
#![feature(slice_patterns)]

#![crate_type="dylib"]
#![crate_name="protobuf_macros"]

extern crate rustc;
extern crate syntax;

extern crate rustc_plugin;
use rustc_plugin::Registry;

mod util;

mod protobuf_init;
use protobuf_init::macro_protobuf_init;

mod protobuf_bind;
use protobuf_bind::macro_protobuf_bind;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("protobuf_init", macro_protobuf_init);
    reg.register_macro("protobuf_bind", macro_protobuf_bind);
}


