#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate bincode;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate wasm_tutorial_shared;

use wasm_bindgen::prelude::*;
use wasm_tutorial_shared::models::{Message, ToDo};

/// Import console.log from the js context
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
/// Safety wrapper around logging that will detect
/// if we should use log or println!
fn print(s: &str) {
    if cfg!(target_arch = "wasm32") {
        log(s);
    } else {
        println!("{}", s);
    }
}

/// Convert a buffer of bincode into a json string
/// This is primarily used for the GetAll message
/// type
#[wasm_bindgen]
pub fn bincode_to_json(buffer: Vec<u8>) -> String {
    print("bincode_to_json");
    if let Ok(msg) = Message::from_bytes(buffer) {
        match msg {
            Message::All(todos) => {
                if let Ok(ser) = serde_json::to_string(&todos) {
                    return ser
                }
            },
            Message::Error(msg) => {
                print(&format!("Error getting todos from msg {}", msg));
            },
            _ => print("Incorrect message type")
        }
    }
    "[]".into()
}

/// Create a new ToDo item with an id of -1 and the action provided, create a Message::Add
/// with the ToDo and serialize it to a buffer
#[wasm_bindgen]
pub fn get_add_message(action: String) -> Vec<u8> {
    print(&format!("get_add_message({})", action));
    let todo = ToDo::new(action);
    let msg = Message::Add(todo);
    print(&format!("sending back message: {:?}", msg));
    msg.to_bytes()
}

/// Use the provided properties to create a Message::Update and serialize
/// it into a buffer
#[wasm_bindgen]
pub fn get_update_message(id: i32, complete: bool, action: String) -> Vec<u8> {
    print(&format!("get_update_message({}, {}, {})", id, complete, action));
    let todo = ToDo{id, complete, action};
    let msg = Message::Update(todo);
    msg.to_bytes()
}

/// Create a Message::Remove with the id provided
/// and serialize it into a buffer
#[wasm_bindgen]
pub fn get_remove_message(id: i32) -> Vec<u8> {
    print(&format!("get_remove_message({})", id));
    let msg = Message::Remove(id);
    msg.to_bytes()
}
/// Create a Message::GetAll and serialize it into a buffer
#[wasm_bindgen]
pub fn get_all_message() -> Vec<u8> {
    print(&format!("get_all_message"));
    Message::GetAll.to_bytes()
}


#[cfg(test)]
mod test {
    use super::{
        ToDo,
        Message,
        bincode_to_json,
        get_add_message,
        get_update_message,
        get_remove_message,
        get_all_message,
        serde_json,
    };

    #[test]
    fn b_to_j() {
        let todos = vec![ToDo{id:10,complete: false,action:"things".into()}, ToDo{id:19,complete:true,action:"stuff".into()}];
        let msg = Message::All(todos.clone());
        let bytes = msg.to_bytes();
        let j = bincode_to_json(bytes);
        let parsed: Vec<ToDo> = serde_json::from_str(&j).expect("Unable to parse json");
        assert_eq!(todos, parsed);
    }
    #[test]
    fn get_add() {
        let action = "Learn back flips";
        let bytes = get_add_message(action.to_string());
        if let Message::Add(todo) = Message::from_bytes(bytes).unwrap() {
            assert_eq!(todo.action, action);
        } else {
            panic!("Incorrect message type");
        }
    }

    #[test]
    fn get_update() {
        let todo = ToDo::new("Things and stuff");
        let bytes = get_update_message(todo.id, todo.complete, todo.action.clone());
        if let Message::Update(changes) = Message::from_bytes(bytes).unwrap() {
            assert_eq!(changes, todo);
        } else {
            panic!("Incorrect message type");
        }
    }

    #[test]
    fn get_remove() {
        let id = 10;
        let bytes = get_remove_message(id);
        if let Message::Remove(parsed) = Message::from_bytes(bytes).unwrap() {
            assert_eq!(parsed, id);
        } else {
            panic!("Incorrect message type");
        }
    }

    #[test]
    fn get_all() {
        let bytes = get_all_message();
        assert_eq!(Message::GetAll, Message::from_bytes(bytes).unwrap());
    }
}