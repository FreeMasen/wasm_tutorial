use std::env::{set_var, var};

extern crate wasm_tutorial_server;
fn main() {
    let port = match var("PORT") {
        Ok(port) => {
            println!("found $PORT env var: {:?}", &port);
            port
        },
        Err(e) => {
            println!("Unable to get $PORT from env {:?}", e);
            let port = String::from("8888");
            set_var("PORT", &port);
            port
        },
    };
    wasm_tutorial_server::start_server(&port);
}