use std::fs::{File};
use std::path::PathBuf;

use bincode::{deserialize_from, serialize_into};
use wasm_bindgen::prelude::*;

type DataResult<T> = Result<T, String>;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ToDo {
    pub id: i32,
    pub complete: bool,
    pub action: String,
}

pub struct Data {
    todos: Vec<ToDo>
}

impl Data {
    pub fn new() -> DataResult<Data> {
        let todos = Self::get_from_file()?;
        Ok(Data {
            todos,
        })
    }

    pub fn get_todos(&self) -> Vec<ToDo> {
        self.todos.clone()
    }

    pub fn add(&mut self, todo: &mut ToDo) -> DataResult<Vec<ToDo>>  {
        if todo.id > -1 {
            Err(format!("An item with the id {} already exists", todo.id))
        } else {
            todo.id = self.todos.len() as i32;
            self.todos.push(todo.to_owned());
            self.save_to_file()
        }
    }

    pub fn update(&mut self, todo: &ToDo) -> DataResult<Vec<ToDo>> {
        if todo.id > -1 {
            self.todos = self.todos.clone().into_iter().map(|t| {
                if t.id == todo.id {
                    todo.to_owned()
                } else {
                    t
                }
            }).collect();
            self.save_to_file()
        } else {
            Err(String::from("An id is required to update an item"))
        }
    }

    pub fn remove(&mut self, id: i32) -> DataResult<Vec<ToDo>> {
        if id < -1 {
            return Err("ID is required to remove an item".into())
        }
        self.todos = self.todos.clone()
            .into_iter()
            .filter(|t| t.id == id)
            .collect();
        self.save_to_file()
    }

    fn save_to_file(&self) -> DataResult<Vec<ToDo>> {
        let f = File::create("data.bincode").map_err(|e| format!("{:?}", e))?;
        match serialize_into(f, &self.todos) {
            Ok(_) => Ok(self.get_todos()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn get_from_file() -> DataResult<Vec<ToDo>> {
        let path = PathBuf::from("data.bincode");
        if !path.exists() {
            Ok(vec!())
        } else {
            let f = File::open("data.bincode").map_err(|e| format!("{:?}", e))?;
            match deserialize_from(&f) {
                Ok(todos) => Ok(todos),
                Err(e) => Err(format!("{}",e))
            }
        }
    }
}

