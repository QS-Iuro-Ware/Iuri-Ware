mod error;
mod games;
mod messages;
mod room;
mod server;
mod session;

pub use crate::error::IuroError;
pub use crate::server::IuroServer;

use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::error;
use rand::random;
use serde_json::{from_str, to_string};
use std::{borrow::Cow, time::Instant};

mod prelude {
    pub use crate::messages::{Response, *};
    pub use crate::room::{Room, RoomSlot};
    pub use crate::session::IuroSession;
    pub use crate::{IuroError, IuroServer};
    pub use crate::games::prelude::*;
    pub use log::{debug, error, info, trace, warn};
}

use crate::prelude::{Response, *};

pub fn handle_text(msg: &str, act: &mut IuroSession, ctx: &mut Ctx) -> Result<(), IuroError> {
    match from_str(&msg)? {
        Command::ListRooms => {
            let future = send(act, ListRooms).map(Response::Rooms);
            spawn(future.into_actor(act), ctx);
        }
        Command::Join(room) => {
            let (id, name) = (act.id, room.clone());
            let future = send(act, Join { id, name }).and_then(|r| r);

            let future = future.into_actor(act).map(move |_: (), act, _| {
                // Caches room locally so we can send messages to it ergonomically
                act.room = Some(room);
                Response::Text(Cow::Borrowed("Joined room"))
            });
            spawn(future, ctx);
        }
        Command::Name(name) => {
            let data = SetUsername {
                user_id: act.id,
                name,
                room: act.room.clone(),
            };
            let future = send(act, data)
                // No message is sent to user in case of success
                .map(|_| None)
                .into_actor(act);
            spawn(future, ctx);
        }
        Command::Message(msg) => {
            if let Some(room) = act.room.clone() {
                let cmd = ChatMessage {
                    id: act.id,
                    msg,
                    room,
                };

                // Send message to `IuroServer` broadcast to user's room
                act.addr.do_send(cmd);
            } else {
                return Err(IuroError::MustJoinRoom);
            }
        }
        Command::Game(games) => {
            let input = UserGameInput {
                id: act.id,
                room: act.room.as_ref().ok_or(IuroError::MustJoinRoom)?.clone(),
                input: games,
            };

            let future = send(act, input)
                .and_then(|r| r)
                // No message is sent to user in case of success
                .map(|_| None)
                .into_actor(act);
            spawn(future, ctx);
        }
    }
    Ok(())
}

type Ctx = ws::WebsocketContext<IuroSession>;

/// Entry point for our route
pub fn iuro_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<IuroServer>>,
) -> Result<HttpResponse, actix_web::Error> {
    let session = IuroSession {
        // This is not ideal since `ThreadRng` is not cached,
        // but it's better than needing an `Actor` to generate an id
        id: random(),
        heartbeat: Instant::now(),
        room: None,
        addr: srv.get_ref().clone(),
    };
    // Upgrades connection to websocket
    ws::start(session, &req, stream)
}

/// Abstracts sending message to `IuroServer` and actix error handling
fn send<M>(act: &mut IuroSession, cmd: M) -> impl Future<Item = M::Result, Error = IuroError>
where
    M: Message + Send + 'static + std::fmt::Debug,
    M::Result: Send,
    IuroServer: Handler<M>,
{
    debug!("Command: {:?}", cmd);
    act.addr.send(cmd).from_err()
}

/// Spawns async task with specified future, sending its result with websocket
///
/// If `None` is passed in `Item` no message is sent on success
fn spawn(
    fut: impl ActorFuture<Item = impl Into<Option<Response>>, Error = IuroError, Actor = IuroSession>
        + 'static,
    ctx: &mut Ctx,
) {
    fut.then(|res, _, ctx| {
        let json = match res {
            Ok(res) => {
                // If `None` is passed no message is sent on success
                if let Some(res) = res.into() {
                    to_string(&res)
                } else {
                    return fut::ok(());
                }
            }
            Err(err) => to_string(&Response::Error(err.to_string())),
        };

        if let Ok(json) = json {
            trace!("Sending: {}", json);
            ctx.text(json);
        } else {
            // This should never happen
            error!("Failed to serialize `Response`");
            debug_assert!(false, "Failed to serialize `Response`");
        }
        fut::ok(())
    })
    .spawn(ctx);
}
