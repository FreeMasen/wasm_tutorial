#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate bincode;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate wasm_tutorial_shared;

use wasm_bindgen::prelude::*;
use wasm_tutorial_shared::models::{Message, ToDo};

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn bincode_to_json(buffer: Vec<u8>) -> String {
    log("bincode_to_json");
    if let Ok(msg) = Message::from_bytes(buffer) {
        match msg {
            Message::All(todos) => {
                if let Ok(ser) = serde_json::to_string(&todos) {
                    return ser
                }
            },
            Message::Error(msg) => {
                log(&format!("Error getting todos from msg {}", msg));
            },
            _ => log("Incorrect message type")
        }
    }
    "[]".into()
}

#[wasm_bindgen]
pub fn get_add_message(action: String) -> Vec<u8> {
    log(&format!("get_add_message({})", action));
    let todo = ToDo::new(-1, false, action);
    let msg = Message::Add(todo);
    log(&format!("sending back message: {:?}", msg));
    msg.to_bytes()
}

#[wasm_bindgen]
pub fn get_update_message(id: f64, complete: bool, action: String) -> Vec<u8> {
    log(&format!("get_update_message({}, {}, {})", id, complete, action));
    let todo = ToDo::new(id as i32, complete, action);
    let msg = Message::Update(todo);
    msg.to_bytes()
}

#[wasm_bindgen]
pub fn get_remove_message(id: i32) -> Vec<u8> {
    log(&format!("get_remove_message({}, {}, {})", id, complete, action));
    let msg = Message::Remove(id);
    msg.to_bytes()
}

#[wasm_bindgen]
pub fn get_all_message() -> Vec<u8> {
    log(&format!("get_all_message"));
    Message::GetAll.to_bytes()
}