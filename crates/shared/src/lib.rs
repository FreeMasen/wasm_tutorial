#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod data;
pub mod messages;