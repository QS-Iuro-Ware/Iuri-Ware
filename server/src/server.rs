//! `IuroServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `IuroServer`.

use crate::{commands::Message, commands::*, error::IuroError};
use actix::prelude::*;
use log::trace;
use std::collections::HashMap;

/// Maps user's random id to its connection
type SessionMap = HashMap<usize, Recipient<Message>>;

/// `IuroServer` manages rooms and is responsible for coordinating them
#[derive(Default)]
pub struct IuroServer {
    unbound_sessions: SessionMap,
    rooms: HashMap<String, SessionMap>,
}

impl IuroServer {
    /// Send message to all users in the room, ignoring full mailboxes
    fn send_message(&self, room: &str, message: &str) -> Result<(), IuroError> {
        if let Some(sessions) = self.rooms.get(room) {
            for addr in sessions.values() {
                // Ignores recipients with a full mailbox
                let _ = addr.do_send(Message(message.to_owned()));
            }
            Ok(())
        } else {
            Err(IuroError::NoRoom(room.to_owned()))
        }
    }

    /// Removes user from all rooms, returning its address, errors if user isn't in any room
    fn leave_all_rooms(&mut self, id: usize) -> Result<Recipient<Message>, IuroError> {
        for sessions in self.rooms.values_mut() {
            if let Some(addr) = sessions.remove(&id) {
                for addr in sessions.values() {
                    // Ignores recipients with a full mailbox
                    let _ = addr.do_send(Message("Someone disconnected".to_owned()));
                }
                return Ok(addr);
            }
        }
        Err(IuroError::AddrNotFound(id))
    }
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for IuroServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        trace!("Websocket connection stablished: id = {}", msg.id);
        self.unbound_sessions.insert(msg.id, msg.addr);
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for IuroServer {
    type Result = Result<(), IuroError>;

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        trace!("Websocket connection closed: id = {}", msg.id);

        if self.unbound_sessions.remove(&msg.id).is_none() {
            let _ = self.leave_all_rooms(msg.id)?;
        }
        Ok(())
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for IuroServer {
    type Result = Result<(), IuroError>;

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) -> Self::Result {
        if let Some(name) = msg.name {
            self.send_message(&msg.room, &format!("{}: {}", name, msg.msg))
        } else {
            self.send_message(&msg.room, &format!("user-{}: {}", msg.id, msg.msg))
        }
    }
}

/// Handler for `ListRooms` message.
impl Handler<ListRooms> for IuroServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        MessageResult(self.rooms.keys().cloned().collect())
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for IuroServer {
    type Result = Result<(), IuroError>;

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) -> Self::Result {
        let Join { id, name } = msg;

        // Remove address
        let addr = if let Some(addr) = self.unbound_sessions.remove(&msg.id) {
            addr
        } else {
            self.leave_all_rooms(id)?
        };

        // Creates room on demand
        self.rooms
            .entry(name)
            .or_insert_with(SessionMap::default)
            .insert(id, addr);
        Ok(())
    }
}

/// Make actor from `IuroServer`
impl Actor for IuroServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}
