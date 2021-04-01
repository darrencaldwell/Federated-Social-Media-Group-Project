use std::{collections::{HashMap, HashSet}, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

use crate::chat::routes;

use actix::prelude::*;

use rand::{Rng, prelude::ThreadRng};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: String,
    pub user_name: String,
    pub addr: Recipient<Message>,
    pub room: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
    pub user_name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: String,
    pub msg: String,
    pub room: String,
}

pub struct ChatServer {
    sessions: HashMap<String, Recipient<Message>>,
    rooms: HashMap<String, HashSet<String>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rng: rand::thread_rng(),
            visitor_count,
        }
    }

    fn send_message(&self, room: &str, message: &str, skip_id: String) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(msg.id.clone(), msg.addr);

        self.rooms
            .entry(msg.room.clone())
            .or_insert_with(HashSet::new)
            .insert(msg.id.clone());

        let join_msg = routes::Message {
            message_type: routes::MessageType::Connect,
            user_id: msg.id.clone(),
            user_name: msg.user_name,
            content: None,
        };

        let join_msg = serde_json::to_string(&join_msg).unwrap();

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message(&msg.room, &join_msg, msg.id);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        let mut rooms: Vec<String> = Vec::new();

        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }

        let leave_msg = routes::Message {
            message_type: routes::MessageType::Disconnect,
            user_id: msg.id.clone(),
            user_name: msg.user_name,
            content: None,
        };

        let leave_msg = serde_json::to_string(&leave_msg).unwrap();

        for room in rooms {
            self.send_message(&room, &leave_msg, msg.id.clone());
        }
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) -> Self::Result {
        self.send_message(&msg.room, msg.msg.as_str(), msg.id.clone());
    }
}
