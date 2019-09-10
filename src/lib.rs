mod commands;
mod error;
mod server;

pub use crate::error::IuroError;
pub use crate::server::IuroServer;

use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::{error, trace};
use rand::random;
use serde::Serialize;
use serde_json::{from_str, to_string};
use std::{time::Duration, time::Instant, fmt::Formatter, fmt, fmt::Debug};

use crate::commands::Commands;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

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

/// Each websocket connection generates a session that exists until it's closed
struct WsIuroSession {
    /// Unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    heartbeat: Instant,
    /// Joined room
    room: Option<String>,
    /// Peer name
    name: Option<String>,
    /// Iuro server
    addr: Addr<server::IuroServer>,
}

impl Debug for WsIuroSession {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("WsIuroSession")
            .field("id", &self.id)
            .field("heartbeat", &self.heartbeat)
            .field("room", &self.room)
            .field("name", &self.name)
            .field("addr", &"Addr<IuroServer>")
            .finish()
    }
}

impl Actor for WsIuroSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with IuroServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // We'll start heartbeat process on session start.
        self.heartbeat(ctx);

        // Register self in iuro's server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsIuroSessionState, state is shared
        // across all routes within application
        let (id, addr) = (self.id, ctx.address().recipient());
        self.addr.do_send(commands::Connect { id, addr });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(commands::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from iuro server, we simply send it to peer websocket
impl Handler<commands::Message> for WsIuroSession {
    type Result = ();

    fn handle(&mut self, msg: commands::Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

#[derive(Serialize)]
enum Responses {
    Rooms(Vec<String>),
    Text(String),
    Error(String),
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
    fut
        .then(|res, _, ctx| {
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

fn handle_text(
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
            let fut = send(act, commands::Join { id: act.id, name: name.clone() })
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

/// WebSocket message handler
impl StreamHandler<ws::Message, ws::ProtocolError> for WsIuroSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                trace!("Websocket Message: {:?}", text);
                if let Err(err) = handle_text(&text, self, ctx) {
                    if let Ok(json) = to_string(&Responses::Error(err.to_string())) {
                        ctx.text(json)
                    } else {
                        error!("Failed to serialize `Responses`");
                    }
                }
            }
            // Web-Browsers don't support built-in Ping/Pong, we must mock it with binary data
            ws::Message::Binary(raw) => {
                if raw == [0x09][..] {
                    trace!("Hearbeat");
                    self.heartbeat = Instant::now();
                } else {
                    error!("Unexpected binary data: {:?}", raw);
                }
            }
            ws::Message::Close(_) => ctx.stop(),
            ws::Message::Ping(msg) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => (),
            ws::Message::Nop => (),
        }
    }
}

impl WsIuroSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let id = self.id;
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            // Check client heartbeats
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                // Heartbeat timed out
                trace!("Websocket Client heartbeat failed, disconnecting!");

                // Notify iuro server
                act.addr.do_send(commands::Disconnect { id });

                // Stop actor
                ctx.stop();
            } else {
                // This doesn't work properly in web browsers
                // (browsers API don't allow you to intercept Ping/Pong)
                ctx.ping("");
            }
        });
    }
}
