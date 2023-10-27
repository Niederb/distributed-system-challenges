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

fn main() {
    let stdin = io::stdin();
    let iterator = stdin.lock().lines();
    let mut initialized = false;
    let mut node_id = "Hello World".to_string();
    let mut current_id = 0;
    let mut messages = HashSet::<usize>::new();
    let mut my_topology = HashMap::<String, Vec<String>>::new();
    for it in iterator {
        let request = it.unwrap();
        let response = if initialized {
            let request: Message<Body<Payload>> = serde_json::from_str(&request).unwrap();
            let response = match &request.body.message_body {
                Payload::Broadcast { message } => {
                    messages.insert(*message);
                    let message_body = Payload::BroadcastOk;
                    let response = create_response(&request, message_body, current_id);
                    serde_json::to_string(&response).unwrap()
                }
                Payload::Read => {
                    let message_body = Payload::ReadOk {
                        messages: messages.clone(),
                    };
                    let response = create_response(&request, message_body, current_id);
                    serde_json::to_string(&response).unwrap()
                }
                Payload::Topology { topology } => {
                    my_topology.extend(topology.clone());
                    let message_body = Payload::TopologyOk;
                    let response = create_response(&request, message_body, current_id);
                    serde_json::to_string(&response).unwrap()
                }
                Payload::Gossip { message } => "".to_string(),
                Payload::ReadOk { .. } => "".to_string(),
                Payload::TopologyOk => "".to_string(),
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
