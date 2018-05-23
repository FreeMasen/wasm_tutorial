use std::fs::{File};
use std::path::PathBuf;

use bincode::{deserialize_from, serialize_into, ErrorKind};
use models::ToDo;

///Custom error type to allow for ? early returns
#[derive(Debug)]
pub struct DataError {
    msg: String
}
/// Custom result type for easier return defs
pub type DataResult<T> = Result<T, DataError>;

/// Our actual data representation
pub struct Data {
    todos: Vec<ToDo>
}

impl Data {
    /// Attemptes to get the stored data from the flat file
    /// If that fails, creates an empty data structure and saves
    /// it to the flat file
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
    /// Get the next id -1 from our list of ToDos
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
    /// Get a clone of our todo list
    pub fn get_todos(&self) -> Vec<ToDo> {
        self.todos.clone()
    }
    /// Add a new item to our todo list (id must be -1)
    pub fn add(&mut self, todo: &ToDo) -> DataResult<Vec<ToDo>>  {
        println!("Add {:?}", todo);
        if todo.id > -1 {
            Err(DataError::new(format!("An item with the id {} already exists", todo.id)))
        } else {
            let adding = ToDo {
                id: self.max_id() + 1,
                complete: todo.complete,
                action: todo.action.clone(),
            };
            self.todos.push(adding);
            self.save_to_file()
        }
    }
    /// Update an existing todo item based on its id
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
            Err(DataError::new("An id is required to update an item"))
        }
    }
    /// Remove any ToDos with the matching id
    pub fn remove(&mut self, id: i32) -> DataResult<Vec<ToDo>> {
        println!("Attempting to remove todo with id {}", id);
        println!("todos: {:?}", &self.todos);
        if id < -1 {
            return Err(DataError::new("ID is required to remove an item"))
        }
        self.todos.retain(|t| t.id != id);
        self.save_to_file()
    }
    /// Save our ToDo List to disk, returning the full list
    /// if successful
    fn save_to_file(&self) -> DataResult<Vec<ToDo>> {
        let f = File::create("data.bincode")?;
        serialize_into(f, &self.todos)?;
        Ok(self.get_todos())
    }
    /// Get the full list of ToDos from disk
    fn get_from_file() -> DataResult<Vec<ToDo>> {
        println!("Data::get_from_file");
        let path = PathBuf::from("data.bincode");
        let f = File::open(path)?;
        let todos = deserialize_from(&f)?;
        Ok(todos)
    }
}

impl DataError {
    /// Construct a new DataError with the msg provided
    fn new(msg: impl Into<String>) -> DataError {
        DataError {
            msg: msg.into()
        }
    }
}

impl From<::std::io::Error> for DataError {
    /// For ? from io errors
    fn from(other: ::std::io::Error) -> DataError {
        DataError::new(format!("{:?}", other))
    }
}
impl From<::std::boxed::Box<ErrorKind>> for DataError {
    /// for ? from bincode errors
    fn from(other: Box<ErrorKind>) -> DataError {
        DataError::new(format!("{:?}", other))
    }
}
impl ::std::fmt::Display for DataError {
    /// required for implemting std::error::Error
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        self.msg.fmt(f)
    }
}

impl ::std::error::Error for DataError {
    fn description(&self) -> &str {
        &self.msg
    }
}