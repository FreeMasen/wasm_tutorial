use std::fs::{File};
use std::path::PathBuf;

use bincode::{deserialize_from, serialize_into};
use models::ToDo;
type DataResult<T> = Result<T, String>;

pub struct Data {
    todos: Vec<ToDo>
}

impl Data {
    pub fn new() -> DataResult<Data> {
        match Self::get_from_file() {
            Ok(todos) => Ok(Data {
                todos,
            }),
            Err(e) => {
                println!("{:?}\n creating new file", e);
                let ret = Data {
                    todos: vec!(),
                };
                ret.save_to_file()?;
                Ok(ret)
            }
        }
        
    }

    fn max_id(&self) -> i32 {
        if self.todos.len() == 0 {
            return 0
        }
        if let Some(max) = self.todos.iter().map(|t| t.id).max() {
            max
        } else {
            0
        }
    }

    pub fn get_todos(&self) -> Vec<ToDo> {
        self.todos.clone()
    }

    pub fn add(&mut self, todo: &mut ToDo) -> DataResult<Vec<ToDo>>  {
        if todo.id > -1 {
            Err(format!("An item with the id {} already exists", todo.id))
        } else {
            todo.id = self.max_id() + 1;
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
        println!("Attempting to remove todo with id {}", id);
        println!("todos: {:?}", &self.todos);
        if id < -1 {
            return Err("ID is required to remove an item".into())
        }
        self.todos = self.todos.clone()
            .into_iter()
            .filter(|t| t.id != id)
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
        println!("Data::get_from_file");
        let path = PathBuf::from("data.bincode");
        if !path.exists() {
            Err(format!("{:?} does not exist", path))
        } else {
            let f = File::open("data.bincode").map_err(|e| format!("{:?}", e))?;
            match deserialize_from(&f) {
                Ok(todos) => Ok(todos),
                Err(e) => Err(format!("{}",e))
            }
        }
    }
}

