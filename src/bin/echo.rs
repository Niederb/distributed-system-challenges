use serde::{Deserialize, Serialize};
use std::io;
use std::io::BufRead;
use distributed_system_challenges::{Message, RequestBody, ResponseBody};

#[derive(Serialize, Deserialize)]
struct Init {
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct InitOk;

#[derive(Serialize, Deserialize)]
struct Echo {
    echo: String,
}

#[derive(Serialize, Deserialize)]
struct EchoOk {
    echo: String,
}

fn main() {
    let stdin = io::stdin();
    let iterator = stdin.lock().lines();
    let mut initialized = false;
    for it in iterator {
        let request = it.unwrap();
        let response = if initialized {
            let request: Message<RequestBody<Echo>> = serde_json::from_str(&request).unwrap();
            let message_body = EchoOk {
                echo: request.body.message_body.echo,
            };
            let body = ResponseBody {
                msg_id: 1,
                type_: "echo_ok".to_string(),
                in_reply_to: request.body.msg_id,
                message_body,
            };
            let response = Message {
                src: request.dest,
                dest: request.src,
                body,
            };
            serde_json::to_string(&response).unwrap()
        } else {
            let request: Message<RequestBody<Init>> = serde_json::from_str(&request).unwrap();
            let message_body = InitOk;
            let body = ResponseBody {
                msg_id: 1,
                type_: "init_ok".to_string(),
                in_reply_to: request.body.msg_id,
                message_body,
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
