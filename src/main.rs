use std::env::{set_var, var, args};

extern crate wasm_tutorial_server;
fn main() {
    let port = if let Some(p) = args().nth(1) {
        println!("passed port arg: {}", &p);
        p
    } else {
        match var("PORT") {
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
        }
    };
    wasm_tutorial_server::start_server(&port);
}