use crate::prelude::*;
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

/// All messages that can be sent to user (serialized)
#[derive(Serialize, Debug)]
pub enum Response {
    /// List of rooms
    Rooms(Vec<String>),
    /// Chat message
    Text(Cow<'static, str>),
    /// User appropriate error message
    Error(String),
    /// Which game is starting
    GameStarted(Game),
    /// Returns map of users and their wins
    GameEnded((&'static str, HashMap<String, usize>)),
}

/// Commands sent from client (to be deserialized)
#[derive(Deserialize, Debug)]
pub enum Command {
    /// Returns all existing rooms -> '"ListRooms"'
    ListRooms,
    /// Inserts user in room, create it if non existant, removes user from the other room (if any), starts game if room is full '{ "Join": <string> }'
    Join(String),
    /// Set user's name, to send with messages '{ "Name": <string> }'
    Name(String),
    /// Message to be multicasted to all users in same room as sender, except the sender '{ "Message": <string> }'
    Message(String),
    /// Game input sent from user { "Game": { "RockPapiuroScissor": "Rock" } }
    Game(GameInput),
}

/// Attach user to its `GameInput`
#[derive(Deserialize, Message, Debug)]
#[rtype("Result<(), IuroError>")]
pub struct UserGameInput {
    pub id: usize,
    pub room: String,
    pub input: GameInput,
}

/// Message to be broadcasted to a room
#[derive(Message, Clone, Debug)]
pub enum Broadcast {
    /// Regular text message, sent to chat
    Text(String),
    /// Same as Text, but with a string literal, avoiding unecessary allocations
    Literal(&'static str),
    /// Game that has just started, with its data
    GameStarted(Game),
    /// Game ended, returns map of users and their winnings
    GameEnded((&'static str, HashMap<String, usize>)),
}

/// Creates new session
#[derive(Message)]
pub struct Connect {
    pub id: usize,
    pub addr: Recipient<Broadcast>,
}

/// Disconnect session
#[derive(Message, Debug)]
#[rtype("Result<(), IuroError>")]
pub struct Disconnect {
    pub id: usize,
}

/// Set user's name
#[derive(Message, Debug)]
#[rtype("Result<(), IuroError>")]
pub struct SetUsername {
    pub user_id: usize,
    pub room: Option<String>,
    pub name: String,
}

/// List available rooms
#[derive(Message, Debug)]
#[rtype("Vec<String>")]
pub struct ListRooms;

/// Sends message to specific room
#[derive(Message, Debug)]
#[rtype("Result<(), IuroError>")]
pub struct ChatMessage {
    pub id: usize,
    pub msg: String,
    pub room: String,
}

/// Join room, if room does not exists create new one, leave other rooms. Starts game if room is full
#[derive(Message, Debug)]
#[rtype("Result<(), IuroError>")]
pub struct Join {
    pub id: usize,
    pub name: String,
}
