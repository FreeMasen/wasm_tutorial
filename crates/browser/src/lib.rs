#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate wasm_tutorial_shared;

use wasm_tutorial_shared::messages::Message;

#[wasm_bindgen]
pub fn serialize(message: Message) -> Vec<u8> {
    match message.to_bytes() {
        Ok(bytes) => bytes,
        Err(e) => vec!()
    }
}

#[wasm_bindgen]
pub fn deserialize(bytes: Vec<u8>) -> Message {
    match Message::from_bytes(&bytes) {
        Ok(msg) => msg,
        Err(e) => Message::for_error(format!("{:?}", e))
    }
}