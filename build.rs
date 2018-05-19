use std::{
    process::{Command},

};

fn main() {
    Command::new("sh")
        .arg("build_wasm.sh")
        .spawn()
        .expect("Unable to execute build script")
        .wait_with_output()
        .expect("Unable to wait");
}