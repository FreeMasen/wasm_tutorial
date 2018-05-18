use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ToDo {
    pub id: i32,
    pub complete: bool,
    pub action: String,
}

impl ToDo {
    pub fn new(id: i32, complete: bool, action: impl Into<String>) -> ToDo {
        ToDo {
            id,
            complete,
            action: action.into()
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    GetAll,
    All(Vec<ToDo>),
    Add(ToDo),
    Update(ToDo),
    Remove(ToDo),
    Error(String),
}


impl Message {
    pub fn for_error(data: String) -> Message {
        Message::Error(data)
    }

    pub fn to_bytes(self) -> Vec<u8> {
        serialize(&self).unwrap_or(vec!())
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Message, String> {
        match deserialize(&bytes) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(format!("{:?}", e))
        }
    }
}

///messages coming from the client
impl Message {
    pub fn get_all() -> Message {
        Message::GetAll
    }

    pub fn add(todo: ToDo) -> Message {
        Message::Add(todo)
    }
    pub fn update(todo: ToDo) -> Message {
        Message::Update(todo)
    }

    pub fn remove(todo: ToDo) -> Message {
        Message::Remove(todo)
    }

    pub fn all(todos: Vec<ToDo>) -> Message {
        Message::All(todos)
    }
}