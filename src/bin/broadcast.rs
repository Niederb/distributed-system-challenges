use distributed_system_challenges::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: HashSet<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
    /*Gossip {
        seen: HashSet<usize>,
    },*/
}

fn main() {
    let stdin = io::stdin();
    let iterator = stdin.lock().lines();
    let mut initialized = false;
    let mut node_id = "Hello World".to_string();
    let mut current_id = 0;
    let mut messages = HashSet::<usize>::new();
    for it in iterator {
        let request = it.unwrap();
        let response = if initialized {
            let request: Message<Body<Payload>> = serde_json::from_str(&request).unwrap();
            let response = match request.body.message_body {
                Payload::Broadcast { message } => {
                    messages.insert(message);
                    let message_body = Payload::BroadcastOk;
                    let body = Body {
                        msg_id: current_id,
                        in_reply_to: Some(request.body.msg_id),
                        message_body,
                    };
                    let response = Message {
                        src: request.dest,
                        dest: request.src,
                        body,
                    };
                    serde_json::to_string(&response).unwrap()
                }
                Payload::Read => {
                    let message_body = Payload::ReadOk {
                        messages: messages.clone(),
                    };
                    let body = Body {
                        msg_id: current_id,
                        in_reply_to: Some(request.body.msg_id),
                        message_body,
                    };
                    let response = Message {
                        src: request.dest,
                        dest: request.src,
                        body,
                    };
                    serde_json::to_string(&response).unwrap()
                }
                Payload::Topology { .. }=> {
                    let message_body = Payload::TopologyOk;
                    let body = Body {
                        msg_id: current_id,
                        in_reply_to: Some(request.body.msg_id),
                        message_body,
                    };
                    let response = Message {
                        src: request.dest,
                        dest: request.src,
                        body,
                    };
                    serde_json::to_string(&response).unwrap()
                }
                Payload::ReadOk { .. } => "".to_string(),
                Payload::TopologyOk  => "".to_string(),
                Payload::BroadcastOk => "".to_string(),
            };
            response
        } else {
            let (response, id) = process_init(&request);
            node_id = id;
            initialized = true;
            response
        };
        println!("{}", response);
        current_id += 1;
    }
}
