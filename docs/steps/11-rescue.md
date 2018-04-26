---
permalink: "/steps/11-rescue.html"
title: "WASM to the rescue"
layout: "post"
prev: "/steps/10.1-the-wrench.html"
---
#### Generate a new library project
```bash
$ cargo new --lib
```

#### bincode_parse.rs
```rust
#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate bincode;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate wasm_tutorial_shared;

use wasm_tutorial_shared::models::{Message, ToDo};
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn bincode_to_json(buffer: Vec<u8>) -> String {
    log("bincode_to_json");
    if let Ok(msg) = Message::from_bytes(buffer.to_vec()) {
        match msg.todos() {
            Ok(todos) => {
                if let Ok(ser) = serde_json::to_string(&todos) {
                    return ser
                }
            },
            Err(e) => {
                log(&format!("Error getting todos from msg {:?}", e));
            }
        }
    }
    "[]".into()
}
#[wasm_bindgen]
pub fn get_add_message(action: String) -> Vec<u8> {
    log(&format!("get_add_message {}", action));
    let todo = ToDo { id: -1, complete: false, action: action };
    let msg = Message::add_client(todo);
    log(&format!("sending back message: {:?}", msg));
    msg.to_bytes()
}

#[wasm_bindgen]
pub fn get_update_message(id: f64, complete: bool, action: String) -> Vec<u8> {
    let todo = ToDo { id: id as i32, complete, action };
    let msg = Message::update_client(todo);
    msg.to_bytes()
}
#[wasm_bindgen]
pub fn get_remove_message(id: f64, complete: bool, action: String) -> Vec<u8> {
    let todo = ToDo { id: id as i32, complete, action };
    let msg = Message::remove_client(todo);
    msg.to_bytes()
}
#[wasm_bindgen]
pub fn get_all_message() -> Vec<u8> {
    let msg = Message::get_all_client();
    msg.to_bytes()
}
```

#### network.ts
```typescript
import ToDo from '../models/todo';
/**
 * The network service, this wraps up our XHR and wasm interactions
 */
export default class Network {
    /**
     * Create's a new Network service
     * @param wasm The WebAssembly module to be used when generating messages
     */
    constructor(
        private wasm,
    ) {}
    /**
     * Get the list of todo items from the server
     */
    async getToDoList(): Promise<ToDo[]> {
        return fetch('/todos')
            .then(res => {
                return res.arrayBuffer()
            })
            .then(buf => this.bufferToArray(buf));
    }
    /**
     * Send a new todo entry to the server and get back a fresh list of todos
     * @param action The text that should appear in the new todo
     */
    async addToDoItem(action: string): Promise<ToDo[]> {
        let body: ArrayBuffer = this.wasm.get_add_message(action)
        console.log('sending new message', body);
        return fetch('/todos', {body, method: 'POST'})
            .then(res => {
                console.log(res)
                return res.arrayBuffer()
            })
            .then(buf => this.bufferToArray(buf));
    }
    /**
     * Send a todo to the server to be updated
     * @param item The item to be updated
     */
    async updateToDoItem(item: ToDo): Promise<ToDo[]> {
        let body: ArrayBuffer = this.wasm.get_update_message(item.id, item.complete, item.action);
        return fetch('/todos', {body, method: 'POST'})
            .then(res => res.arrayBuffer())
            .then(buf => this.bufferToArray(buf));
    }
    /**
     * Send a todo to the server to be removed
     * @param item The todo to be removed
     */
    async removeToDoItem(item: ToDo): Promise<ToDo[]> {
        let body: ArrayBuffer = this.wasm.get_remove_message(item.id, item.complete, item.action);
        return fetch('/todos', {body, method: 'POST'})
        .then(res => res.arrayBuffer())
        .then(buf => this.bufferToArray(buf));
    }
    /**
     * Convert the server's response from raw bytes into an array of ToDo items
     * @param buf The bytes to be converted by the wasm module
     */
    bufferToArray(buf: ArrayBuffer): ToDo[] {
        let json = this.wasm.bincode_to_json(new Uint8Array(buf));
        return JSON.parse(json);
    }
}
```