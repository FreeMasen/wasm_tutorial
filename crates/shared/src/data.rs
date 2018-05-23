use std::fs::{File};
use std::path::{Path};

use bincode::{deserialize_from, serialize_into, ErrorKind};
use models::ToDo;

///Custom error type to allow for ? early returns
#[derive(Debug)]
pub struct DataError {
    pub msg: String
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
        Self::get_or_create("data.bincode")
    }
    /// This function will wrap our first call to the disk
    /// This will allow for easily changing the path
    /// at a later time
    fn get_or_create(path: impl AsRef<Path>) -> DataResult<Data> {
        match Self::get_from_file(path) {
            Ok(todos) => Ok(Data {
                todos
            }),
            Err(e) => {
                println!("{:?}\n creating new file", e);
                let ret = Data {
                    todos: vec!(),
                };
                ret.save_and_get()?;
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
            self.save_and_get()
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
            self.save_and_get()
        } else {
            Err(DataError::new("An id is required to update an item"))
        }
    }
    /// Remove any ToDos with the matching id
    pub fn remove(&mut self, id: i32) -> DataResult<Vec<ToDo>> {
        println!("Attempting to remove todo with id {}", id);
        if id < 0 {
            return Err(DataError::new("ID is required to remove an item"))
        }
        self.todos.retain(|t| t.id != id);
        self.save_and_get()
    }
    /// Save our ToDo List to disk, returning the full list
    /// if successful
    fn save_and_get(&self) -> DataResult<Vec<ToDo>> {
        self.save("data.bincode")?;
        Ok(self.get_todos())
    }
    fn save(&self, path: impl AsRef<Path>) -> DataResult<()> {
        let f = File::create(path)?;
        serialize_into(f, &self.todos)?;
        Ok(())
    }
    /// Get the full list of ToDos from disk
    fn get_from_file(path: impl AsRef<Path>) -> DataResult<Vec<ToDo>> {
        println!("Data::get_from_file");
        let f = File::open(path)?;
        let todos = deserialize_from(&f)?;
        Ok(todos)
    }
}

impl DataError {
    /// Construct a new DataError with the msg provided
    pub fn new(msg: impl Into<String>) -> DataError {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_test() {
        let file_name = "create.bincode";
        Data::get_or_create(&file_name).expect("Error getting data from disk");
        remove_test_file(&file_name)
    }
    #[test]
    fn invalid_file_test() {
        let file_name = "invalid_file.bincode";
        {
            use std::fs::File;
            use std::io::Write;
            let mut f = File::create(&file_name).expect("Unable to create invalid file");
            let _ = f.write_all("ERROR".as_bytes()).expect("Unabel to write invalid test");
        }
        let _ = Data::get_or_create(&file_name).expect("Unable to construct Data from invalid file");
        remove_test_file(&file_name);
    }
    #[test]
    fn existing_file() {
        let file_name = "existing_file.bincode";
        let todos = mock_todos();
        {
            use std::fs::File;
            let f = File::create(&file_name).expect("Unable to create invalid file");
            use bincode::serialize_into;
            serialize_into(f, &todos).expect("Unable to ser mock todos");
            println!("Successfully created {}", &file_name);
        }
        let d = Data::get_or_create(&file_name).expect("Unable to get data");
        assert_eq!(d.todos, todos);
        remove_test_file(&file_name);
    }

    #[test]
    fn add_test() {
        let file_name = "add_test.bincode";
        let mut d = Data::get_or_create(&file_name).unwrap();
        let new_todo = ToDo::new(-1, false, "Take a walk");
        d.add(&new_todo).expect("Unable to add new todo");
        assert!(d.todos.iter().any(|t| t.action == new_todo.action));
        remove_test_file(&file_name);
    }
    #[test]
    fn update_test() {
        let file_name = "update_test.bincode";
        let mut d = Data::get_or_create(&file_name).unwrap();
        d.todos = mock_todos();
        d.update(&ToDo::new(0, true, "Walk the dog")).expect("Unable to update todo");
        assert!(d.todos.iter().any(|t| t.action == "Walk the dog" && t.id == 0 && t.complete == true));
        assert!(d.todos != mock_todos());
        remove_test_file(&file_name);
    }
    #[test]
    fn remove_test() {
        let file_name = "remove_test.bincode";
        let mut d = Data::get_or_create(&file_name).unwrap();
        d.todos = mock_todos();
        remove_test_file(&file_name);
        d.remove(0).expect("Unable to remove todo with ID 0");
        assert!(d.todos.len() < mock_todos().len());
        assert!(d.todos.iter().all(|t| t.id != 0));
        remove_test_file(&file_name);
    }

    #[test]
    fn save_test() {
        let file_name = "save_test.bincode";
        let mut d = Data::get_or_create(&file_name).unwrap();
        d.todos = mock_todos();
        d.save(&file_name).expect("unable to save Data");
        let d2 = Data::get_or_create(&file_name).unwrap();
        assert_eq!(d.todos, d2.todos);
        remove_test_file(&file_name);
    }
    #[test]
    fn add_fail() {
        let file_name = "add_fail.bincode";
        let mut d = Data::get_or_create(&file_name).unwrap();
        match d.add(&ToDo::new(6, false, "")) {
            Ok(_) => {
                remove_test_file(&file_name);
                panic!("Added todo with id != -1")
            },
            Err(_) => ()
        }
        remove_test_file(&file_name);
    }
    #[test]
    fn remove_fail() {
        let file_name = "remove_fail.bincode";
        let mut d = Data::get_or_create(&file_name).unwrap();
        if let Ok(_) = d.remove(-1) {
            remove_test_file(&file_name);
            panic!("Removed successfully with id == -1");
        }
        remove_test_file(&file_name)
    }

    #[test]
    fn update_fail() {
        let file_name = "update_fail.bincode";
        let mut d = Data::get_or_create(&file_name).unwrap();
        match d.update(&ToDo::new(-1, false, "")) {
            Ok(_) => panic!("Updated successfully with an id == -1"),
            _ => ()
        }
    }

    fn mock_todos() -> Vec<ToDo> {
        vec![
            ToDo::new(0, false, "Walk the dog"),
            ToDo::new(1, false, "Buy new couch"),
            ToDo::new(2, true, "Watch Luther"),
            ToDo::new(3, true, "Build Website"),
        ]
    }
    fn remove_test_file(file_name: impl AsRef<Path>) {
        use std::fs::remove_file;
        match remove_file(file_name) {
            Ok(_) => (),
            Err(e) => eprintln!("Error removing test file {:?}", e),

        }
    }
}