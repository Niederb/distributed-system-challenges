use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message<Body> {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Serialize, Deserialize)]
pub struct Body<MessageBody> {
    pub msg_id: u32,
    pub in_reply_to: Option<u32>,
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
    let request: Message<Body<InitPayload>> = serde_json::from_str(&request).unwrap();
    match &request.body.message_body {
        InitPayload::Init { node_id, node_ids } => {
            let message_body = InitPayload::InitOk;
            let response = create_response(&request, message_body, 0);
            (
                serde_json::to_string(&response).unwrap(),
                node_id.to_string(),
            )
        }
        InitPayload::InitOk => ("".to_string(), "".to_string()),
    }
}

pub fn create_response<Payload>(
    request: &Message<Body<Payload>>,
    payload: Payload,
    current_msg_id: u32,
) -> Message<Body<Payload>> {
    let body = Body {
        msg_id: current_msg_id,
        in_reply_to: Some(request.body.msg_id),
        message_body: payload,
    };
    let response = Message {
        src: request.dest.clone(),
        dest: request.src.clone(),
        body,
    };
    response
}
