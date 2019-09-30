mod commands;
mod error;
mod server;
mod session;

pub use crate::error::IuroError;
pub use crate::server::IuroServer;

use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::error;
use rand::random;
use serde::Serialize;
use serde_json::{from_str, to_string};
use std::{time::Instant, borrow::Cow};

use crate::{commands::Commands, session::IuroSession};

#[derive(Serialize)]
enum Responses {
    Rooms(Vec<String>),
    Text(Cow<'static, str>),
    Error(String),
}

type Ctx = ws::WebsocketContext<IuroSession>;

pub fn handle_text(msg: &str, act: &mut IuroSession, ctx: &mut Ctx) -> Result<(), IuroError> {
    match from_str(&msg)? {
        Commands::ListRooms => {
            // Ask `IuroServer` to list all available rooms and define response
            let fut = send(act, commands::ListRooms).map(Responses::Rooms);

            // Spawn async task to serialize and send response
            spawn(fut.into_actor(act), ctx);
        }
        Commands::Join(room) => {
            // Ask `IuroServer` to add insert user to specified room (receiving all its new broadcasts)
            // and leave any other, creating a new room if non existant
            let (id, name) = (act.id, room.clone());

            // Join returns a result over the send result, so we have to "unwrap" it
            let fut = send(act, commands::Join { id, name }).and_then(|r| r);

            let fut = fut.into_actor(act).map(move |_: (), act, _| {
                // Caches room locally so we can send messages to it ergonomically
                act.room = Some(room);
                Responses::Text(Cow::Borrowed("Joined room"))
            });

            // Spawn async task to serialize and send the response
            spawn(fut, ctx);
        }
        Commands::Name(name) => act.name = Some(name),
        Commands::Message(msg) => {
            if let Some(room) = act.room.clone() {
                let cmd = commands::ClientMessage {
                    id: act.id,
                    name: act.name.clone(),
                    msg,
                    room,
                };

                // Send message to `IuroServer` broadcast to user's room (except the user)
                act.addr.do_send(cmd);
            } else {
                return Err(IuroError::MustJoinRoom);
            }
        }
    }
    Ok(())
}

/// Entry point for our route
pub fn iuro_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::IuroServer>>,
) -> Result<HttpResponse, actix_web::Error> {
    let session = IuroSession {
        // This is not ideal since `ThreadRng` is not cached,
        // but it's better than needing an `Actor` to generate an id
        id: random(),
        heartbeat: Instant::now(),
        room: None,
        name: None,
        addr: srv.get_ref().clone(),
    };
    // Upgrades connection to websocket
    ws::start(session, &req, stream)
}

/// Abstracts sending message to `IuroServer` and actix error handling
fn send<M>(act: &mut IuroSession, cmd: M) -> impl Future<Item = M::Result, Error = IuroError>
where
    M: Message + Send + 'static,
    M::Result: Send,
    IuroServer: Handler<M>,
{
    act.addr.send(cmd).from_err()
}

/// Spawns async task with specified future, sending its result in websocket
fn spawn(
    fut: impl ActorFuture<Item = Responses, Error = IuroError, Actor = IuroSession> + 'static,
    ctx: &mut Ctx,
) {
    fut.then(|res, _, ctx| {
        let json = match res {
            Ok(res) => to_string(&res),
            Err(err) => to_string(&Responses::Error(err.to_string())),
        };

        if let Ok(json) = json {
            ctx.text(json);
        } else {
            // Oh crap
            // This should be unreachable
            error!("Failed to serialize `Responses`");
        }
        fut::ok(())
    })
    .spawn(ctx);
}
