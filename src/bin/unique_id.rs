use serde::{Deserialize, Serialize};
use std::io;
use std::io::BufRead;
use distributed_system_challenges::*;

#[derive(Serialize, Deserialize)]
struct Generate {
    dummy_value: Option<u32>
}

#[derive(Serialize, Deserialize)]
struct GenerateOk {
    id: String,
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
            let request: Message<RequestBody<Generate>> = serde_json::from_str(&request).unwrap();
            let id = format!("{}-{}", node_id, current_id);
            current_id += 1;
            let message_body = GenerateOk { id };
            let body = ResponseBody {
                msg_id: 1,
                type_: "generate_ok".to_string(),
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
            node_id = request.body.message_body.node_id;
            let message_body = InitOk::new();
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
