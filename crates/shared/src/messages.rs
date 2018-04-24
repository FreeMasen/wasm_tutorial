

use bincode::{serialize, deserialize};
use models::{ToDo, Message, MessageType};

impl Message {
    pub fn for_error(data: String) -> Message {
        Message {
            kind: MessageType::Error,
            data: data.as_bytes().to_vec()
        }
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

    pub fn todos(&self) -> Result<Vec<ToDo>, String> {
        match deserialize(&self.data) {
            Ok(todos) => Ok(todos),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

///messages coming from the client
impl Message {
    pub fn get_all_client() -> Message {
        Message {
            kind: MessageType::GetAll,
            data: vec!(),
        }
    }

    pub fn add_client(todo: ToDo) -> Message {
        match serialize(&vec![todo]) {
            Ok(bytes) => Message {
                    kind: MessageType::Add,
                    data: bytes
                },
            Err(e) => Self::for_error(format!("{:?}", e)),
        }
    }
    pub fn update_client(todo: ToDo) -> Message {
        match serialize(&vec![todo]) {
            Ok(bytes) => Message { 
                kind: MessageType::Update,
                data: bytes,
            },
            Err(e) => Self::for_error(format!("{:?}", e)),
        }
    }

    pub fn remove_client(todo: ToDo) -> Message {
        match serialize(&vec![todo]) {
            Ok(bytes) => Message {
                kind: MessageType::Remove,
                data: bytes
            },
            Err(e) => Self::for_error(format!("{:?}", e))
        }
    }
}

//messages coming from the server
impl Message {
    pub fn get_all_server(todos: &Vec<ToDo>) -> Message {
        match serialize(todos) {
            Ok(bytes) => {
                println!("successfully serialized data");
                Message {
                    kind: MessageType::GetAll,
                    data: bytes,
                }
            },
            Err(e) => {
                println!("error serialaizing data {:?}", e);
                Self::for_error(format!("{:?}", e))
            }
        }
    }

    pub fn add_server(todos: Result<Vec<ToDo>, String>) -> Message {
        match todos {
            Ok(todos) => {
                match serialize(&todos) {
                    Ok(bytes) => Message {
                        kind: MessageType::Add,
                        data: bytes
                    },
                    Err(e) => Self::for_error(format!("{:?}", e))
                }
            },
            Err(e) => Self::for_error(format!("{:?}", e))
        }
    }
    pub fn update_server(todos: Result<Vec<ToDo>, String>) -> Message {
        match todos {
            Ok(todos) => match serialize(&todos) {
                Ok(bytes) => Message {
                    kind: MessageType::Update,
                    data: bytes
                },
                Err(e) => Self::for_error(format!("{:?}", e))
            },
            Err(e) => Self::for_error(format!("{:?}", e))
        }
        
    }

    pub fn remove_server(todos: Result<Vec<ToDo>, String>) -> Message {
        match todos {
            Ok(todos) => match serialize(&todos) {
                Ok(bytes) => Message {
                    kind: MessageType::Remove,
                    data: bytes
                },
                Err(e) => Self::for_error(format!("{:?}", e))
            },
            Err(e) => Self::for_error(format!("{:?}", e))
        }
    }
}