use distributed_system_challenges::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
struct Broadcast {
    message: u32,
}

#[derive(Serialize, Deserialize)]
struct BroadcastOk{
    dummy_value: Option<u32>,
}

impl BroadcastOk {
    pub fn new() -> Self {
        Self { dummy_value: None }
    }
}

#[derive(Serialize, Deserialize)]
struct Read {
    dummy_value: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct ReadOk {
    messages: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct Topology {
    topology: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct TopologyOk {
    dummy_value: Option<u32>,
}

impl TopologyOk {
    pub fn new() -> Self {
        Self { dummy_value: None }
    }
}

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
            println!("{}", request);
            let request: Message<Payload> = serde_json::from_str(&request).unwrap();
            
            let body = BroadcastOk::new();
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
