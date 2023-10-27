use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message<Body> {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody<MessageBody> {
    pub msg_id: u32,
    #[serde(flatten)]
    pub message_body: MessageBody,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseBody<MessageBody> {
    pub msg_id: u32,
    pub in_reply_to: u32,
    #[serde(flatten)]
    pub message_body: MessageBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum InitPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

pub fn process_init(request: &str) -> (String, String) {
    let request: Message<RequestBody<InitPayload>> = serde_json::from_str(&request).unwrap();
    match request.body.message_body {
        InitPayload::Init { node_id, node_ids } => {
            let message_body = InitPayload::InitOk;
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
            (serde_json::to_string(&response).unwrap(), node_id)
        }
        InitPayload::InitOk => ("".to_string(), "".to_string()),
    }
}
