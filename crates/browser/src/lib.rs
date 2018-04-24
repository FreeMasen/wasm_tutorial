#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate wasm_tutorial_shared;

use wasm_tutorial_shared::messages::Message;
use wasm_tutorial_shared::data::ToDo as ServerToDo;
#[derive(Serialize)]
#[wasm_bindgen]
pub struct ToDo {
    id: i32,
    complete: bool,
    action: String,
}

impl From<ServerToDo> for ToDo {
    fn from(todo: ServerToDo) -> ToDo {
        ToDo {
            id: todo.id,
            complete: todo.complete,
            action: todo.action,
        }
    }
}

#[wasm_bindgen]
pub fn bincode_to_json(buffer: Box<[u8]>) -> String {
    if let Ok(msg) = Message::from_bytes(buffer.to_vec()) {
        let todos: Vec<ToDo> = msg.todos().into_iter().map(ToDo::from).collect();
        if let Ok(ser) = serde_json::to_string(&todos) {
            return ser
        }
    }
    String::new()
}
#[wasm_bindgen]
pub fn get_add_message(action: String) -> Vec<u8> {
    let todo = ServerToDo { id: -1, complete: false, action: action };
    let msg = Message::add_client(&todo);
    msg.to_bytes()
}

#[wasm_bindgen]
pub fn get_update_message(id: f64, complete: bool, action: String) -> Vec<u8> {
    let todo = ServerToDo { id: id as i32, complete, action };
    let msg = Message::update_client(&todo);
    msg.to_bytes()
}
#[wasm_bindgen]
pub fn get_remote_message(id: f64, complete: bool, action: String) -> Vec<u8> {
    let todo = ServerToDo { id: id as i32, complete, action };
    let msg = Message::remove_client(&todo);
    msg.to_bytes()
}
#[wasm_bindgen]
pub fn get_all_message() -> Vec<u8> {
    let msg = Message::get_all_client();
    msg.to_bytes()
}