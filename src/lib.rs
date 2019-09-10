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
use std::time::Instant;

use crate::{commands::Commands, session::WsIuroSession};

#[derive(Serialize)]
enum Responses {
    Rooms(Vec<String>),
    Text(String),
    Error(String),
}

pub fn handle_text(
    msg: &str,
    act: &mut WsIuroSession,
    ctx: &mut ws::WebsocketContext<WsIuroSession>,
) -> Result<(), IuroError> {
    match from_str(&msg)? {
        Commands::ListRooms => {
            let fut = send(act, commands::ListRooms).map(Responses::Rooms);
            spawn(fut.into_actor(act), ctx);
        }
        Commands::Join(name) => {
            let fut = send(
                act,
                commands::Join {
                    id: act.id,
                    name: name.clone(),
                },
            )
            .and_then(|r| r)
            .into_actor(act)
            .map(move |_: (), act, _| {
                act.room = Some(name);
                Responses::Text("Joined room".to_owned())
            });
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
    ws::start(
        WsIuroSession {
            // This is not ideal since `ThreadRng` is not cached,
            // but it's better than needing an `Actor` to generate id
            id: random(),
            heartbeat: Instant::now(),
            room: None,
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

fn send<M>(act: &mut WsIuroSession, cmd: M) -> impl Future<Item = M::Result, Error = IuroError>
where
    M: Message + Send + 'static,
    M::Result: Send,
    IuroServer: Handler<M>,
{
    act.addr.send(cmd).from_err()
}

fn spawn(
    fut: impl ActorFuture<Item = Responses, Error = IuroError, Actor = WsIuroSession> + 'static,
    ctx: &mut ws::WebsocketContext<WsIuroSession>,
) {
    fut.then(|res, _, ctx| {
        let json = match res {
            Ok(res) => to_string(&res),
            Err(err) => to_string(&Responses::Error(err.to_string())),
        };

        if let Ok(json) = json {
            ctx.text(json);
        } else {
            error!("Failed to serialize `Responses`");
        }
        fut::ok(())
    })
    .spawn(ctx);
}
