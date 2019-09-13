use actix::prelude::*;
use actix_web_actors::ws;
use log::{debug, error, trace};
use serde_json::to_string;
use std::{fmt, fmt::Debug, fmt::Formatter, time::Duration, time::Instant};

use crate::{commands, handle_text, IuroServer, Responses};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Each websocket connection generates a session that exists until it's closed
pub struct WsIuroSession {
    /// Unique session id
    pub id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop the connection.
    pub heartbeat: Instant,
    /// Room users is authenticated to
    pub room: Option<String>,
    /// Peer name
    pub name: Option<String>,
    /// Iuro server's address
    pub addr: Addr<IuroServer>,
}

/// WebSocket message handler
impl StreamHandler<ws::Message, ws::ProtocolError> for WsIuroSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                debug!("Websocket Message: {:?}", text);
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

/// Handle messages from iuro server, we simply send it to peer websocket
impl Handler<commands::Message> for WsIuroSession {
    type Result = ();

    fn handle(&mut self, msg: commands::Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
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
