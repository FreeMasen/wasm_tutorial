use std::env::{var};

extern crate wasm_tutorial_server;
fn main() {
    let port = match var("PORT") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error getting port {:?}", e);
            "8888".into()
        }
    };
    wasm_tutorial_server::start_server(&port);
}