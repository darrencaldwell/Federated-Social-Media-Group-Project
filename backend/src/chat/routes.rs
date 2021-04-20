use std::time::{Duration, Instant};

use sqlx::MySqlPool;

use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web, get};
use actix_web::http::HeaderName;

use actix_web_actors::ws;

use serde::Serialize;

use crate::{chat::server, id_extractor::UserId};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Serialize)]
pub enum MessageType {
    Connect,
    Disconnect,
    Message,
    Server,
    UserList,
}

#[derive(Serialize)]
pub struct Message {
    pub message_type: MessageType,
    pub user_id: String,
    pub user_name: String,
    pub content: Option<String>,
}

struct WsChatSession {
    id: String,
    hb: Instant,
    room: String,
    name: String,
    addr: Addr<server::ChatServer>,
}

impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(server::Disconnect { user_name: act.name.clone(), id: act.id.clone() });
                ctx.stop();

                return
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(server::Connect { 
                user_name: self.name.clone(),
                id: self.id.clone(),
                addr: addr.recipient(),
                room: self.room.clone() }
            )
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(()) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { user_name: self.name.clone(), id: self.id.clone() });
        Running::Stop
    }
}

impl Handler<server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return
            }
            Ok(msg) => msg
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();

                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/help" => {
                            let msg = Message {
                                message_type: MessageType::Server,
                                user_id: self.id.clone(),
                                user_name: self.name.clone(),
                                content: Some("List of commands: /help".to_string())
                            };
                            ctx.text(serde_json::to_string(&msg).unwrap());
                        },
                        "/ping" => {
                            let msg = Message {
                                message_type: MessageType::Server,
                                user_id: self.id.clone(),
                                user_name: self.name.clone(),
                                content: Some("pong".to_string())
                            };
                            ctx.text(serde_json::to_string(&msg).unwrap());
                        },
                        _ => {
                            let msg = Message {
                                message_type: MessageType::Server,
                                user_id: self.id.clone(),
                                user_name: self.name.clone(),
                                content: Some("Not a valid command, type /help".to_string())
                            };
                            ctx.text(serde_json::to_string(&msg).unwrap());
                        },
                    }
                } else {
                    let msg = Message {
                        message_type: MessageType::Message,
                        user_id: self.id.clone(),
                        user_name: self.name.clone(),
                        content: Some(m.to_owned()),
                    };

                    self.addr.do_send(server::ClientMessage {
                        id: self.id.clone(),
                        msg: serde_json::to_string(&msg).unwrap(),
                        room: self.room.clone(),
                    })
                }
            }
            ws::Message::Binary(_) => println!("unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop()
            }
            ws::Message::Nop => (),
        }
    }
}

#[get("/chat/forums/{id}")]
pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
    pool: web::Data<MySqlPool>,
    web::Path(id): web::Path<String>,
    UserId(user_id): UserId,
) -> Result<HttpResponse, Error> {

    let user_name = sqlx::query!("SELECT username FROM users WHERE user_id = ?", user_id)
        .fetch_one(pool.as_ref())
        .await.unwrap()
        .username.unwrap();

    let res = ws::start(
        WsChatSession {
            id: user_id,
            hb: Instant::now(),
            room: id,
            name: user_name,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    if res.is_ok() {
        // add protocol from request to response
        let mut resp = res.unwrap();
        let token = req.headers().get("sec-websocket-protocol").unwrap();
        resp.headers_mut().insert(HeaderName::from_static("sec-websocket-protocol"), token.clone());
        return Ok(resp)
    } else {
        return res
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(chat_route);
}
