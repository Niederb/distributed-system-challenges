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
    Gossip {
        message: usize,
    },
}

fn gossip_to_neighbors() {
    
}

fn main() {
    let stdin = io::stdin();
    let iterator = stdin.lock().lines();
    let mut initialized = false;
    let mut node = Node::new("".to_string());
    let mut messages = HashSet::<usize>::new();
    let mut my_topology = HashMap::<String, Vec<String>>::new();
    for it in iterator {
        let request = it.unwrap();
        if initialized {
            let request: Message<Body<Payload>> = serde_json::from_str(&request).unwrap();
            match &request.body.message_body {
                Payload::Broadcast { message } => {
                    //messages.insert(*message);
                    let message_body = Payload::BroadcastOk;
                    let response = create_response(&request, message_body, node.current_msg_id);
                    node.send_message(response);
                    if !messages.contains(&message) {
                        messages.insert(*message);
                        let neighbors = &my_topology[&node.node_id];
                        for n in neighbors {
                            if *n != request.src {
                                let payload = Payload::Gossip { message: *message };
                                let body = Body {
                                    msg_id: node.current_msg_id,
                                    in_reply_to: Some(request.body.msg_id),
                                    message_body: payload,
                                };
                                let response = Message {
                                    src: node.node_id.to_string(),
                                    dest: n.to_string(),
                                    body,
                                };
                                node.send_message(response);
                            }
                        }
                    }
                }
                Payload::Read => {
                    let message_body = Payload::ReadOk {
                        messages: messages.clone(),
                    };
                    let response = create_response(&request, message_body, node.current_msg_id);
                    node.send_message(response);
                }
                Payload::Topology { topology } => {
                    my_topology.extend(topology.clone());
                    let message_body = Payload::TopologyOk;
                    let response = create_response(&request, message_body, node.current_msg_id);
                    node.send_message(response);
                }
                Payload::Gossip { message } => {
                    if !messages.contains(&message) {
                        messages.insert(*message);
                        let neighbors = &my_topology[&node.node_id];
                        for n in neighbors {
                            if *n != request.src {
                                let payload = Payload::Gossip { message: *message };
                                let body = Body {
                                    msg_id: node.current_msg_id,
                                    in_reply_to: Some(request.body.msg_id),
                                    message_body: payload,
                                };
                                let response = Message {
                                    src: node.node_id.to_string(),
                                    dest: n.to_string(),
                                    body,
                                };
                                node.send_message(response);
                            }
                        }
                    }
                },
                Payload::ReadOk { .. } => (),
                Payload::TopologyOk => (),
                Payload::BroadcastOk => (),
            };
        } else {
            node = process_init(&request);
            initialized = true;
        };
    }
}
