use std::{
    env::{home_dir},
    process::{Command},
};

fn main() {
    if !cfg!(debug_assertions) {
        Command::new("sh")
            .arg("build_wasm.sh")
            .spawn()
            .expect("Unable to execute build_wasm")
            .wait_with_output()
            .expect("Unable to execute build script");

    }
}