use bincode::{serialize, deserialize};
/// This represents a single todo item it will have a unique ID,
/// a flag whether or not it has been completed and
/// the action that should be taken to complete
/// ```
/// # use wasm_tutorial_shared::models::*;
/// let walk_dog = ToDo {
///     id: 1,
///     complete: false,
///     action: "Walk the dog".into()
/// };
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ToDo {
    pub id: i32,
    pub complete: bool,
    pub action: String,
}

impl ToDo {
    /// construct a new ToDo item
    /// ```
    /// # use wasm_tutorial_shared::models::*;
    /// let walk_dog = ToDo::new(1, false, "Walk the dog");
    /// ```
    pub fn new(id: i32, complete: bool, action: impl Into<String>) -> ToDo {
        ToDo {
            id,
            complete,
            action: action.into()
        }
    }
}

/// A representation of our messages traveling between
/// the server and the client
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Message {
    /// A message from the client to the server to
    /// get all of the ToDo items
    GetAll,
    /// A message from the server to the client with
    /// all of the todo items include.
    /// This will be returned with every request from the client
    All(Vec<ToDo>),
    /// A message from the client to add a new ToDo to the list
    Add(ToDo),
    /// A message from the client to update an existing ToDo
    Update(ToDo),
    /// A message from the client to remove an existing ToDo
    /// with a matching id
    Remove(i32),
    /// A message from the server with an Error message
    Error(String),
}

impl Message {
    /// Constructor for our Error type
    /// ```
    /// # use wasm_tutorial_shared::models::*;
    /// let e = Message::for_error("Message");
    /// ```
    pub fn for_error(data: impl Into<String>) -> Message {
        Message::Error(data.into())
    }
    /// A convenience function for serializing a message
    /// to Bincode
    pub fn to_bytes(self) -> Vec<u8> {
        serialize(&self).unwrap_or(vec!())
    }
    /// A convenience function for deserializing a message
    /// from Bincode
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Message, String> {
        match deserialize(&bytes) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(format!("{:?}", e))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialization() {
        assert_eq!(Message::GetAll,
            Message::from_bytes(Message::GetAll.to_bytes()).unwrap());
        let al = Message::All(vec![ToDo::new(0, false, "")]);
        assert_eq!(al.clone(), Message::from_bytes(al.to_bytes()).unwrap());
        let ad = Message::Add(ToDo::new(-1,false,""));
        assert_eq!(ad.clone(), Message::from_bytes(ad.to_bytes()).unwrap());
        let u = Message::Update(ToDo::new(10,true,"junk"));
        assert_eq!(u.clone(), Message::from_bytes(u.to_bytes()).unwrap());
        let r = Message::Remove(9);
        assert_eq!(r.clone(), Message::from_bytes(r.to_bytes()).unwrap());
        let e = Message::for_error("Error message");
        assert_eq!(Message::Error(String::from("Error message")), Message::from_bytes(e.to_bytes()).unwrap());
    }
    #[test]
    fn new_todo() {
        let nt = ToDo::new(10,false,"junk");
        let t = ToDo { id: 10, complete: false, action: "junk".into()};
        assert_eq!(nt, t);
    }
}