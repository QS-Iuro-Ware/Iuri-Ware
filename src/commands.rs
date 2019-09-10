use crate::IuroError;
use actix::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum Commands {
    ListRooms,
    Join(String),
    Name(String),
    Message(String),
}

/// Iuro server sends this messages to session
#[derive(Message)]
pub struct Message(pub String);

/// New session is created
#[derive(Message)]
pub struct Connect {
    pub id: usize,
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype("Result<(), IuroError>")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message to specific room
#[derive(Message)]
#[rtype("Result<(), IuroError>")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Client's name
    pub name: Option<String>,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

/// List of available rooms
#[derive(Message)]
#[rtype("Vec<String>")]
pub struct ListRooms;

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype("Result<(), IuroError>")]
pub struct Join {
    /// Client id
    pub id: usize,
    /// Room name
    pub name: String,
}
