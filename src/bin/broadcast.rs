use distributed_system_challenges::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        seen: HashSet<usize>,
    },
}

fn main() {
    let stdin = io::stdin();
    let iterator = stdin.lock().lines();
    let mut initialized = false;
    let mut node_id = "Hello World".to_string();
    let mut current_id = 0;
    for it in iterator {
        let request = it.unwrap();
        let response = if initialized {
            let request: Message<Payload> = serde_json::from_str(&request).unwrap();

            let body = Payload::BroadcastOk;
            let response = Message {
                src: request.dest,
                dest: request.src,
                body,
            };
            serde_json::to_string(&response).unwrap()
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
