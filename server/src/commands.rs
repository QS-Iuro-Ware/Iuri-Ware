use crate::IuroError;
use actix::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Copy, Clone, Message, Debug)]
pub enum RockPapiuroScissorInput {
    Rock,
    Papiuro,
    Scissor,
}

#[derive(Deserialize, Message)]
#[rtype("Result<(), IuroError>")]
pub struct UserGameInput {
    pub user_id: usize,
    pub room: String,
    pub input: GameInput
}

#[derive(Deserialize, Message)]
pub enum GameInput {
    RockPapiuroScissor(RockPapiuroScissorInput),
    //WheresIuro((f32, f32)),
}

pub enum GameInputs {
    RockPapiuroScissor(HashMap<usize, RockPapiuroScissorInput>),
    //WheresIuro(HashMap<usize, (f32, f32)>),
}

#[derive(Serialize, Copy, Clone)]
pub enum Games {
    RockPapiuroScissor,
    //WheresIuro,
}

/// Commands sent from client (to be deserialized)
#[derive(Deserialize)]
pub enum Commands {
    /// Returns all existing rooms -> '"ListRooms"'
    ListRooms,
    /// Inserts user in room, create it if non existant, removes user from the other room (if any) '{ "Join": <string> }'
    Join(String),
    /// Set user's name, to send with messages '{ "Name": <string> }'
    Name(String),
    /// Message to be multicasted to all users in same room as sender, except the sender '{ "Message": <string> }'
    Message(String),
    Game(GameInput),
}

/// Message to be passed from Iuro's server to client
#[derive(Message, Clone)]
pub enum Message {
    Text(String),
    GameStarted(Games),
    // user_id -> wins
    GameEnded(HashMap<usize, usize>),
}

/// Create new session
#[derive(Message)]
pub struct Connect {
    pub id: usize,
    pub addr: Recipient<Message>,
}

/// Disconnect session
#[derive(Message)]
#[rtype("Result<(), IuroError>")]
pub struct Disconnect {
    pub id: usize,
}

/// Sends message to specific room
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

/// List available rooms
#[derive(Message)]
#[rtype("Vec<String>")]
pub struct ListRooms;

/// Join room, if room does not exists create new one, leave other rooms.
#[derive(Message)]
#[rtype("Result<(), IuroError>")]
pub struct Join {
    /// Client id
    pub id: usize,
    /// Room name
    pub name: String,
}
