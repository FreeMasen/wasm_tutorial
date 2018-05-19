use std::{
    process::{Command},

};

fn main() {
    if !cfg!(debug_assertions) {
        Command::new("~/.cargo/bin/rustup")
            .arg("install")
            .arg("nightly")
            .spawn()
            .expect("Unable to spawn nightly install")
            .wait_with_output()
            .expect("Unable to execute nightly install");
        Command::new("~/.cargo/bin/rustup")
            .arg("target")
            .arg("add")
            .arg("wasm32-unknown-unknown")
            .expect("Unable to execute target add")
            .unwrap()
            .wait_with_output()
            .expect("Unable to execute wasm target");
        Command::new("~/.cargo/bin/cargo")
            .arg("install")
            .arg("wasm-bindgen")
            .spawn()
            .expect("Unable to install wasm-bindgen")
            .wait_with_output()
            .expect("Unable to execute install wasm-bindgen");
        Command::new("sh")
            .arg("build_wasm.sh")
            .spawn()
            .expect("Unable to execute build_wasm")
            .wait_with_output()
            .expect("Unable to execute build script");
    }
}