use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SessionResult {
    pub access_token: String,
}

//#[derive(Deserialize, Serialize, Debug)]
//pub struct ConversationResponse {
//    message: Message,
//    conversation_id: Option<String>,
//    error: Option<String>,
//}
//
//#[derive(Deserialize, Serialize, Debug)]
//pub struct Message {
//    id: String,
//    content: MessageContent,
//}
//
//#[derive(Deserialize, Serialize, Debug)]
//pub struct MessageContent {
//    content_type: String,
//    parts: Vec<String>,
//}
