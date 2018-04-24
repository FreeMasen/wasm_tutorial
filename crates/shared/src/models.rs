#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ToDo {
    pub id: i32,
    pub complete: bool,
    pub action: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub kind: MessageType,
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageType {
    GetAll,
    Add,
    Update,
    Remove,
    Error,
}
