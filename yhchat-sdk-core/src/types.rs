use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMsgRequest {
    #[serde(rename = "recvId")]
    pub recv_id: String,
    #[serde(rename = "recvType")]
    pub recv_type: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub content: SendContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendContent {
    pub text: Option<String>,
    #[serde(rename = "imageKey")]
    pub image_key: Option<String>,
    #[serde(rename = "fileKey")]
    pub file_key: Option<String>,
    #[serde(rename = "videoKey")]
    pub video_key: Option<String>,
    pub buttons: Option<Vec<SendButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendButton {
    pub text: String,
    #[serde(rename = "actionType")]
    pub action_type: i32,
    pub url: Option<String>,
    pub value: Option<String>,
}

impl SendMsgRequest {
    pub fn text(recv_type: &str, recv_id: &str, text: &str) -> Self {
        Self {
            recv_id: recv_id.to_string(),
            recv_type: recv_type.to_string(),
            content_type: "text".to_string(),
            content: SendContent {
                text: Some(text.to_string()),
                image_key: None,
                file_key: None,
                video_key: None,
                buttons: None,
            },
        }
    }

    pub fn markdown(recv_type: &str, recv_id: &str, text: &str) -> Self {
        Self {
            recv_id: recv_id.to_string(),
            recv_type: recv_type.to_string(),
            content_type: "markdown".to_string(),
            content: SendContent {
                text: Some(text.to_string()),
                image_key: None,
                file_key: None,
                video_key: None,
                buttons: None,
            },
        }
    }

    pub fn image(recv_type: &str, recv_id: &str, image_key: &str) -> Self {
        Self {
            recv_id: recv_id.to_string(),
            recv_type: recv_type.to_string(),
            content_type: "image".to_string(),
            content: SendContent {
                text: None,
                image_key: Some(image_key.to_string()),
                file_key: None,
                video_key: None,
                buttons: None,
            },
        }
    }

    pub fn file(recv_type: &str, recv_id: &str, file_key: &str) -> Self {
        Self {
            recv_id: recv_id.to_string(),
            recv_type: recv_type.to_string(),
            content_type: "file".to_string(),
            content: SendContent {
                text: None,
                image_key: None,
                file_key: Some(file_key.to_string()),
                video_key: None,
                buttons: None,
            },
        }
    }

    pub fn video(recv_type: &str, recv_id: &str, video_key: &str) -> Self {
        Self {
            recv_id: recv_id.to_string(),
            recv_type: recv_type.to_string(),
            content_type: "video".to_string(),
            content: SendContent {
                text: None,
                image_key: None,
                file_key: None,
                video_key: Some(video_key.to_string()),
                buttons: None,
            },
        }
    }

    pub fn html(recv_type: &str, recv_id: &str, text: &str) -> Self {
        Self {
            recv_id: recv_id.to_string(),
            recv_type: recv_type.to_string(),
            content_type: "html".to_string(),
            content: SendContent {
                text: Some(text.to_string()),
                image_key: None,
                file_key: None,
                video_key: None,
                buttons: None,
            },
        }
    }

    pub fn add_button(&mut self, text: &str, action_type: i32, url: Option<&str>, value: Option<&str>) {
        let button = SendButton {
            text: text.to_string(),
            action_type,
            url: url.map(|s| s.to_string()),
            value: value.map(|s| s.to_string()),
        };
        self.content.buttons.get_or_insert_with(Vec::new).push(button);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMsgResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SendMsgData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMsgData {
    #[serde(rename = "messageInfo")]
    pub message_info: Option<MessageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInfo {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "recvId")]
    pub recv_id: String,
    #[serde(rename = "recvType")]
    pub recv_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMsgBatchRequest {
    #[serde(rename = "recvIds")]
    pub recv_ids: Vec<String>,
    #[serde(rename = "recvType")]
    pub recv_type: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub content: SendContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMsgBatchResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SendMsgBatchData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMsgBatchData {
    pub list: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMsgRequest {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "recvId")]
    pub recv_id: String,
    #[serde(rename = "recvType")]
    pub recv_type: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub content: EditContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditContent {
    pub text: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "fileUrl")]
    pub file_url: Option<String>,
    pub buttons: Option<Vec<SendButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMsgResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<EditMsgData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMsgData {
    #[serde(rename = "messageInfo")]
    pub message_info: Option<MessageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallMsgRequest {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "chatType")]
    pub chat_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallMsgResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "chatType")]
    pub chat_type: String,
    #[serde(rename = "msgId")]
    pub msg_id: Option<String>,
    pub r#before: Option<String>,
    pub after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<MessagesData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesData {
    pub list: Option<Vec<MessagesItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesItem {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "sendTime")]
    pub send_time: i64,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "chatType")]
    pub chat_type: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub content: MessagesContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesContent {
    pub text: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    #[serde(rename = "imageName")]
    pub image_name: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "fileUrl")]
    pub file_url: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: Option<i64>,
    pub etag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBoardRequest {
    #[serde(rename = "recvId")]
    pub recv_id: Option<String>,
    #[serde(rename = "recvType")]
    pub recv_type: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBoardResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisBoardRequest {
    #[serde(rename = "recvId")]
    pub recv_id: Option<String>,
    #[serde(rename = "recvType")]
    pub recv_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisBoardResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadRequest {
    #[serde(rename = "filePath")]
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<UploadData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadData {
    #[serde(rename = "imageKey")]
    pub image_key: Option<String>,
    #[serde(rename = "videoKey")]
    pub video_key: Option<String>,
    #[serde(rename = "fileKey")]
    pub file_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSendRequest {
    #[serde(rename = "recvId")]
    pub recv_id: String,
    #[serde(rename = "recvType")]
    pub recv_type: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSendResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SendMsgData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMsgVo {
    pub version: Option<String>,
    pub header: EventHeader,
    pub event: EventContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHeader {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "eventTime")]
    pub event_time: i64,
    #[serde(rename = "eventType")]
    pub event_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContent {
    pub time: Option<i64>,
    #[serde(rename = "chatId")]
    pub chat_id: Option<String>,
    #[serde(rename = "chatType")]
    pub chat_type: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub nickname: Option<String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    #[serde(rename = "settingJson")]
    pub setting_json: Option<String>,
    pub sender: Option<EventSender>,
    pub chat: Option<EventChat>,
    pub message: Option<EventMessage>,
    pub button: Option<EventButton>,
    #[serde(rename = "recvId")]
    pub recv_id: Option<String>,
    #[serde(rename = "recvType")]
    pub recv_type: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventSender {
    #[serde(rename = "senderId")]
    pub sender_id: Option<String>,
    #[serde(rename = "senderType")]
    pub sender_type: Option<String>,
    #[serde(rename = "senderUserLevel")]
    pub sender_user_level: Option<String>,
    #[serde(rename = "senderNickname")]
    pub sender_nickname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventChat {
    #[serde(rename = "chatId")]
    pub chat_id: Option<String>,
    #[serde(rename = "chatType")]
    pub chat_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventButton {
    #[serde(rename = "msgId")]
    pub msg_id: Option<String>,
    #[serde(rename = "buttonId")]
    pub button_id: Option<String>,
    #[serde(rename = "buttonValue")]
    pub button_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessage {
    #[serde(rename = "msgId")]
    pub msg_id: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "sendTime")]
    pub send_time: Option<i64>,
    #[serde(rename = "chatId")]
    pub chat_id: Option<String>,
    #[serde(rename = "chatType")]
    pub chat_type: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub content: Option<EventMessageContent>,
    #[serde(rename = "instructionId")]
    pub instruction_id: Option<i32>,
    #[serde(rename = "instructionName")]
    pub instruction_name: Option<String>,
    #[serde(rename = "commandId")]
    pub command_id: Option<i32>,
    #[serde(rename = "commandName")]
    pub command_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventMessageContent {
    pub text: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    #[serde(rename = "imageName")]
    pub image_name: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "fileUrl")]
    pub file_url: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: Option<i64>,
    pub etag: Option<String>,
    #[serde(rename = "formJson")]
    pub form_json: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}