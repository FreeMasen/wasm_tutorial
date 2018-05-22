use std::env::{var};

extern crate wasm_tutorial_server;
fn main() {
    match var("PORT") {
        Ok(p) => wasm_tutorial_server::start_server(&p),
        Err(e) => eprintln!("Error getting port {:?}", e)
    }
}