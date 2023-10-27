use distributed_system_challenges::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
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
            let request: Message<RequestBody<Payload>> = serde_json::from_str(&request).unwrap();
            match request.body.message_body {
                Payload::Echo { echo } => {
                    let message_body = Payload::EchoOk { echo };
                    let body = ResponseBody {
                        msg_id: 1,
                        in_reply_to: request.body.msg_id,
                        message_body,
                    };
                    let response = Message {
                        src: request.dest,
                        dest: request.src,
                        body,
                    };
                    serde_json::to_string(&response).unwrap()
                }
                Payload::EchoOk { echo } => "".to_string(),
            }
        } else {
            let (response, id) = process_init(&request);
            node_id = id;
            initialized = true;
            response
        };
        println!("{}", response);
    }
}
