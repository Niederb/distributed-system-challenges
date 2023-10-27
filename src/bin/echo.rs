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
struct Echo {
    #[serde(rename = "type")]
    type_: String,
    msg_id: u32,
    echo: String,
}

#[derive(Serialize, Deserialize)]
struct EchoOk {
    #[serde(rename = "type")]
    type_: String,
    msg_id: u32,
    in_reply_to: u32,
    echo: String,
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
    for it in iterator {
        let request = it.unwrap();
        let response = if initialized {
            let request: Message<Echo> = serde_json::from_str(&request).unwrap();
            let body = EchoOk {
                type_: "echo_ok".to_string(),
                msg_id: 1,
                in_reply_to: request.body.msg_id,
                echo: request.body.echo,
            };
            let response = Message {
                src: request.dest,
                dest: request.src,
                body,
            };
            serde_json::to_string(&response).unwrap()
        } else {
            let request: Message<Init> = serde_json::from_str(&request).unwrap();
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
