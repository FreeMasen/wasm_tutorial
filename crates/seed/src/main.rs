extern crate wasm_tutorial_shared;
use wasm_tutorial_shared::data::*;
use wasm_tutorial_shared::models::ToDo;

fn main() {
    let mut data = Data::new().expect("Failed to create data");
    for mut todo in get_todo_list() {
        match data.add(&mut todo) {
            Ok(_list) => println!("Successfully added todo item {}", &todo.action),
            Err(e) => println!("Error adding todo item {:?}", e),
        }
    }
    for todo in data.get_todos() {
        println!("ToDo {}: {}", &todo.id, &todo.action)
    }
}

fn get_todo_list() -> Vec<ToDo> {
    vec![
        ToDo {
            id: -1,
            complete: true,
            action: "Write wasm tutorial server".into(),
        },
        ToDo {
            id: -1,
            complete: true,
            action: "Write wasm totorial client".into(),
        },
        ToDo {
            id: -1,
            complete: true,
            action: "Write wasm totorial outline".into(),
        },
        ToDo {
            id: -1,
            complete: false,
            action: "Present wasm totorial".into()
        },
    ]
}