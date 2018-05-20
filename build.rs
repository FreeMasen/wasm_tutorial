use std::{
    env::{home_dir},
    process::{Command},
};

fn main() {
    if !cfg!(debug_assertions) {
        let home = if let Some(p) = home_dir() {
            p
        } else {
            panic!("Unable to get homedir")
        };
        let base_path = home.join(".cargo/bin/");
        Command::new(base_path.join("rustup"))
            .arg("install")
            .arg("nightly")
            .spawn()
            .expect("Unable to spawn nightly install")
            .wait_with_output()
            .expect("Unable to execute nightly install");
        Command::new(base_path.join("rustup"))
            .arg("target")
            .arg("add")
            .arg("wasm32-unknown-unknown")
            .spawn()
            .expect("Unable to execute target add")
            .wait_with_output()
            .expect("Unable to execute wasm target");
        Command::new(base_path.join("cargo"))
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