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

pub struct Node {
    pub node_id: String,
    pub current_msg_id: u32,
}

impl Node {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            current_msg_id: 0,
        }
    }

    pub fn send_message<Payload: Serialize>(&mut self, message: Message<Body<Payload>>) {
        let message = serde_json::to_string(&message).unwrap();
        println!("{}", message);
        self.current_msg_id += 1;
    }
}

pub fn process_init(request: &str) -> Node {
    let request: Message<Body<InitPayload>> = serde_json::from_str(request).unwrap();
    match &request.body.message_body {
        InitPayload::Init { node_id, .. } => {
            let message_body = InitPayload::InitOk;
            let response = create_response(&request, message_body, 0);
            let mut n = Node::new(node_id.to_string());
            n.send_message(response);
            n
        }
        InitPayload::InitOk => Node::new("".to_string()),
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
    Message {
        src: request.dest.clone(),
        dest: request.src.clone(),
        body,
    }
}
