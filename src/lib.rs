use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message<Body> {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody<MessageBody> {
    #[serde(rename = "type")]
    pub type_: String,
    pub msg_id: u32,
    #[serde(flatten)]
    pub message_body: MessageBody,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseBody<MessageBody> {
    #[serde(rename = "type")]
    pub type_: String,
    pub msg_id: u32,
    pub in_reply_to: u32,
    #[serde(flatten)]
    pub message_body: MessageBody,
}

