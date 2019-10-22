//! `IuroServer` is an actor. It manages user connections. And available rooms.
//! Peers communicate through `IuroServer`.

use crate::{prelude::*, room::JoinResult};
use actix::prelude::*;
use std::collections::HashMap;

/// Manages connections and rooms, coordinates them
#[derive(Default)]
pub struct IuroServer {
    unbound_sessions: HashMap<usize, RoomSlot>,
    rooms: HashMap<String, Room>,
}

impl IuroServer {
    /// Send message to all users in the room, ignoring full mailboxes
    fn send_message(&self, room: &str, message: &Broadcast) -> Result<(), IuroError> {
        debug!("Broadcasting: {:?}", message);

        if let Some(room) = self.rooms.get(room) {
            for slot in room.sessions().values() {
                // Ignores recipients with a full mailbox
                let _ = slot.recipient.do_send(message.clone());
            }
            Ok(())
        } else {
            Err(IuroError::NoRoom(room.to_owned()))
        }
    }

    /// Removes user from all rooms, returning their address, errors if user isn't in any room
    fn leave_all_rooms(&mut self, id: usize) -> Result<RoomSlot, IuroError> {
        let mut ret = None;
        let mut remove_room = None;

        for (name, room) in self.rooms.iter_mut() {
            if let Some(slot) = room.remove_session(id) {
                debug!("User {} left room {}", id, name);

                // Must stop current game (if any)
                room.reset_game();

                // Must delete room if empty
                if room.sessions().is_empty() {
                    remove_room = Some(name.clone());
                }

                let msg = Broadcast::Literal("Someone disconnected");
                for slot in room.sessions().values() {
                    // Ignores recipients with a full mailbox
                    let _ = slot.recipient.do_send(msg.clone());
                }

                ret = Some(slot);
                break;
            }
        }

        if let Some(room) = remove_room {
            trace!("Deleting room: {}", room);
            self.rooms.remove(&room);
        }

        if let Some(slot) = ret {
            Ok(slot)
        } else {
            Err(IuroError::AddrNotFound(id))
        }
    }
}

impl Handler<Connect> for IuroServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        trace!("Websocket connection stablished: id = {}", msg.id);
        let slot = RoomSlot {
            recipient: msg.addr,
            name: format!("user-{}", msg.id % 0xFFF),
            wins: 0,
        };
        self.unbound_sessions.insert(msg.id, slot);
    }
}

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

impl Handler<ChatMessage> for IuroServer {
    type Result = Result<(), IuroError>;

    fn handle(&mut self, msg: ChatMessage, _: &mut Context<Self>) -> Self::Result {
        let name = self
            .rooms
            .get_mut(&msg.room)
            // This should never happen
            .ok_or_else(|| IuroError::NoRoom(msg.room.clone()))?
            .sessions()
            .get(&msg.id)
            // This should never happen
            .ok_or(IuroError::AddrNotFound(msg.id))?
            .name
            .clone();
        let broadcast = Broadcast::Text(format!("{}: {}", name, msg.msg));
        self.send_message(&msg.room, &broadcast)
    }
}

impl Handler<UserGameInput> for IuroServer {
    type Result = Result<(), IuroError>;

    fn handle(&mut self, input: UserGameInput, _: &mut Context<Self>) -> Self::Result {
        let room = self
            .rooms
            .get_mut(&input.room)
            .ok_or_else(|| IuroError::NoRoom(input.room.clone()))?;
        let wins = room.update(input.id, input.input)?;

        if !wins.is_empty() {
            debug!("Game ended: {:?}", wins);
            let game = room.start_game();
            self.send_message(&input.room, &Broadcast::GameEnded(wins))?;
            self.send_message(&input.room, &Broadcast::GameStarted(game))?;
        }
        Ok(())
    }
}

impl Handler<ListRooms> for IuroServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        MessageResult(self.rooms.keys().cloned().collect())
    }
}

impl Handler<SetUsername> for IuroServer {
    type Result = Result<(), IuroError>;

    fn handle(&mut self, set: SetUsername, _: &mut Context<Self>) -> Self::Result {
        if let Some(room) = set.room {
            self.rooms
                .get_mut(&room)
                // This should never happen
                .ok_or_else(|| IuroError::NoRoom(room.clone()))?
                .sessions_mut()
                .get_mut(&set.user_id)
                // This should never happen
                .ok_or(IuroError::AddrNotFound(set.user_id))?
                .name = set.name;
        } else if let Some(slot) = self.unbound_sessions.get_mut(&set.user_id) {
            slot.name = set.name;
        } else {
            return Err(IuroError::AddrNotFound(set.user_id));
        }
        Ok(())
    }
}

impl Handler<Join> for IuroServer {
    type Result = Result<(), IuroError>;

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) -> Self::Result {
        let Join { id, name } = msg;

        // Remove room slot
        let slot = if let Some(slot) = self.unbound_sessions.remove(&msg.id) {
            slot
        } else {
            self.leave_all_rooms(id)?
        };

        // Creates room on demand
        let join = self
            .rooms
            .entry(name.clone())
            .or_insert_with(Room::default)
            .join(id, slot);

        match join {
            JoinResult::NewGame(game) => self.send_message(&name, &Broadcast::GameStarted(game))?,
            JoinResult::NoGame => {}
            JoinResult::Full => return Err(IuroError::FullRoom(name.clone())),
        }

        Ok(())
    }
}

impl Actor for IuroServer {
    type Context = Context<Self>;
}
