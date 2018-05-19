use std::{
    process::{Command},

};

fn main() {
    if !cfg!(debug_assertions) {
        Command::new("~/.cargo/bin/rustup")
            .arg("install")
            .arg("nightly")
            .spawn()
            .expect("Unable to execute nightly install")
            .wait_with_output()
            .expect("Unable to wai");
        Command::new("~/.cargo/bin/rustup")
            .arg("target")
            .arg("add")
            .arg("wasm32-unknown-unknown")
            .expect("Unable to execute wasm target")
            .wait_with_output()
            .expect("unable to wait");
        Command::new("~/.cargo/bin/cargo")
            .arg("install")
            .arg("wasm-bindgen")
            .expect("Unable to execute install wasm-bindgen")
            .wait_with_output()
            .expect("Unable to wait")
        Command::new("sh")
            .arg("build_wasm.sh")
            .spawn()
            .expect("Unable to execute build script")
            .wait_with_output()
            .expect("Unable to wait");
    }
}