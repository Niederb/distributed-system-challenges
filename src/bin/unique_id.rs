use serde::{Deserialize, Serialize};
use std::io;
use std::io::BufRead;

#[derive(Serialize, Deserialize)]
struct Init {
    #[serde(rename = "type")]
    type_: String,
    msg_id: u32,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct InitOk {
    #[serde(rename = "type")]
    type_: String,
    in_reply_to: u32,
}

#[derive(Serialize, Deserialize)]
struct Generate {
    #[serde(rename = "type")]
    type_: String,
    msg_id: u32,
}

#[derive(Serialize, Deserialize)]
struct GenerateOk {
    #[serde(rename = "type")]
    type_: String,
    msg_id: u32,
    in_reply_to: u32,
    id: String,
}

#[derive(Serialize, Deserialize)]
struct Message<Body> {
    src: String,
    dest: String,
    body: Body,
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
            let request: Message<Generate> = serde_json::from_str(&request).unwrap();
            let id = format!("{}-{}", node_id, current_id);
            current_id += 1;
            let body = GenerateOk {
                type_: "generate_ok".to_string(),
                msg_id: 1,
                in_reply_to: request.body.msg_id,
                id: id
            };
            let response = Message {
                src: request.dest,
                dest: request.src,
                body,
            };
            serde_json::to_string(&response).unwrap()
        } else {
            let request: Message<Init> = serde_json::from_str(&request).unwrap();
            node_id = request.body.node_id;
            let body = InitOk {
                type_: "init_ok".to_string(),
                in_reply_to: request.body.msg_id,
            };
            let response = Message {
                src: request.dest,
                dest: request.src,
                body,
            };
            initialized = true;
            serde_json::to_string(&response).unwrap()
        };
        println!("{}", response);
    }
}
