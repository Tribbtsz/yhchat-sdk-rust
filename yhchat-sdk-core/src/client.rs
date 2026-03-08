use crate::constants::BASE_URL;
use crate::types::*;
use reqwest::Client;
use thiserror::Error;
use serde_json::Error as JsonError;

#[derive(Error, Debug)]
pub enum YhChatError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] JsonError),
    #[error("API error: code={0}, msg={1}")]
    ApiError(i32, String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct YhChatClient {
    client: Client,
    token: String,
    base_url: String,
}

impl YhChatClient {
    pub fn new(token: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();
        Self {
            client,
            token: token.to_string(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub fn with_base_url(token: &str, base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();
        Self {
            client,
            token: token.to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub async fn send_msg(&self, req: SendMsgRequest) -> Result<SendMsgResponse, YhChatError> {
        let url = format!("{}/bot/send?token={}", self.base_url, self.token);
        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await?;
        let result: SendMsgResponse = response.json().await?;
        if result.code != 1 {
            return Err(YhChatError::ApiError(result.code, result.msg));
        }
        Ok(result)
    }

    pub async fn send_msg_text(&self, recv_type: &str, recv_id: &str, text: &str) -> Result<SendMsgResponse, YhChatError> {
        let req = SendMsgRequest::text(recv_type, recv_id, text);
        self.send_msg(req).await
    }

    pub async fn send_msg_markdown(&self, recv_type: &str, recv_id: &str, text: &str) -> Result<SendMsgResponse, YhChatError> {
        let req = SendMsgRequest::markdown(recv_type, recv_id, text);
        self.send_msg(req).await
    }

    pub async fn send_msg_image(&self, recv_type: &str, recv_id: &str, image_key: &str) -> Result<SendMsgResponse, YhChatError> {
        let req = SendMsgRequest::image(recv_type, recv_id, image_key);
        self.send_msg(req).await
    }

    pub async fn send_msg_file(&self, recv_type: &str, recv_id: &str, file_key: &str) -> Result<SendMsgResponse, YhChatError> {
        let req = SendMsgRequest::file(recv_type, recv_id, file_key);
        self.send_msg(req).await
    }

    pub async fn send_msg_video(&self, recv_type: &str, recv_id: &str, video_key: &str) -> Result<SendMsgResponse, YhChatError> {
        let req = SendMsgRequest::video(recv_type, recv_id, video_key);
        self.send_msg(req).await
    }

    pub async fn send_msg_html(&self, recv_type: &str, recv_id: &str, text: &str) -> Result<SendMsgResponse, YhChatError> {
        let req = SendMsgRequest::html(recv_type, recv_id, text);
        self.send_msg(req).await
    }

    pub async fn send_msg_batch(&self, recv_type: &str, recv_ids: Vec<&str>, content_type: &str, content: SendContent) -> Result<SendMsgBatchResponse, YhChatError> {
        let url = format!("{}/bot/batch_send?token={}", self.base_url, self.token);
        let req = SendMsgBatchRequest {
            recv_ids: recv_ids.iter().map(|s| s.to_string()).collect(),
            recv_type: recv_type.to_string(),
            content_type: content_type.to_string(),
            content,
        };
        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await?;
        let result: SendMsgBatchResponse = response.json().await?;
        if result.code != 1 {
            return Err(YhChatError::ApiError(result.code, result.msg));
        }
        Ok(result)
    }

    pub async fn edit_msg(&self, req: EditMsgRequest) -> Result<EditMsgResponse, YhChatError> {
        let url = format!("{}/bot/edit?token={}", self.base_url, self.token);
        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await?;
        let result: EditMsgResponse = response.json().await?;
        if result.code != 1 {
            return Err(YhChatError::ApiError(result.code, result.msg));
        }
        Ok(result)
    }

    pub async fn recall_msg(&self, req: RecallMsgRequest) -> Result<RecallMsgResponse, YhChatError> {
        let url = format!("{}/bot/recall?token={}", self.base_url, self.token);
        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await?;
        let result: RecallMsgResponse = response.json().await?;
        if result.code != 1 {
            return Err(YhChatError::ApiError(result.code, result.msg));
        }
        Ok(result)
    }

    pub async fn messages(&self, req: MessagesRequest) -> Result<MessagesResponse, YhChatError> {
        let mut url = format!(
            "{}/bot/messages?token={}&chat-id={}&chat-type={}",
            self.base_url, self.token, req.chat_id, req.chat_type
        );
        if let Some(msg_id) = req.msg_id {
            url.push_str(&format!("&message-id={}", msg_id));
        }
        if let Some(before) = req.r#before {
            url.push_str(&format!("&before={}", before));
        }
        if let Some(after) = req.after {
            url.push_str(&format!("&after={}", after));
        }
        let response = self.client.get(&url).send().await?;
        let result: MessagesResponse = response.json().await?;
        if result.code != 1 {
            return Err(YhChatError::ApiError(result.code, result.msg));
        }
        Ok(result)
    }

    pub async fn set_board(&self, req: SetBoardRequest) -> Result<SetBoardResponse, YhChatError> {
        let url = if req.recv_id.is_none() && req.recv_type.is_none() {
            format!("{}/bot/board-all?token={}", self.base_url, self.token)
        } else {
            format!("{}/bot/board?token={}", self.base_url, self.token)
        };
        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await?;
        let result: SetBoardResponse = response.json().await?;
        if result.code != 1 {
            return Err(YhChatError::ApiError(result.code, result.msg));
        }
        Ok(result)
    }

    pub async fn set_board_all(&self, content_type: &str, content: &str) -> Result<SetBoardResponse, YhChatError> {
        let req = SetBoardRequest {
            recv_id: None,
            recv_type: None,
            content_type: content_type.to_string(),
            content: content.to_string(),
        };
        self.set_board(req).await
    }

    pub async fn set_board_to_user(&self, recv_id: &str, recv_type: &str, content_type: &str, content: &str) -> Result<SetBoardResponse, YhChatError> {
        let req = SetBoardRequest {
            recv_id: Some(recv_id.to_string()),
            recv_type: Some(recv_type.to_string()),
            content_type: content_type.to_string(),
            content: content.to_string(),
        };
        self.set_board(req).await
    }

    pub async fn dis_board(&self, req: DisBoardRequest) -> Result<DisBoardResponse, YhChatError> {
        let url = if req.recv_id.is_none() && req.recv_type.is_none() {
            format!("{}/bot/board-all-dismiss?token={}", self.base_url, self.token)
        } else {
            format!("{}/bot/board-dismiss?token={}", self.base_url, self.token)
        };
        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await?;
        let result: DisBoardResponse = response.json().await?;
        if result.code != 1 {
            return Err(YhChatError::ApiError(result.code, result.msg));
        }
        Ok(result)
    }

    pub async fn dis_board_all(&self) -> Result<DisBoardResponse, YhChatError> {
        let req = DisBoardRequest {
            recv_id: None,
            recv_type: None,
        };
        self.dis_board(req).await
    }

    pub async fn dis_board_to_user(&self, recv_id: &str, recv_type: &str) -> Result<DisBoardResponse, YhChatError> {
        let req = DisBoardRequest {
            recv_id: Some(recv_id.to_string()),
            recv_type: Some(recv_type.to_string()),
        };
        self.dis_board(req).await
    }

    pub async fn upload(&self, file_type: &str, file_path: &str) -> Result<UploadResponse, YhChatError> {
        let url = format!("{}/{}/upload?token={}", self.base_url, file_type, self.token);
        let file_content = std::fs::read(file_path)?;
        let file_name = std::path::Path::new(file_path).file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();
        let file_type_owned = file_type.to_string();
        
        let mime_type = match file_type {
            "image" => "image/png",
            "video" => "video/mp4",
            "file" => "application/octet-stream",
            _ => "application/octet-stream",
        };
        
        let part = reqwest::multipart::Part::bytes(file_content)
            .file_name(file_name)
            .mime_str(mime_type)?;
        let form = reqwest::multipart::Form::new()
            .text("extra", "")
            .part(file_type_owned, part);
        
        let response = self.client.post(&url).multipart(form).send().await?;
        let body = response.text().await?;
        let result: UploadResponse = serde_json::from_str(&body)?;
        if result.code != 1 {
            return Err(YhChatError::ApiError(result.code, result.msg));
        }
        Ok(result)
    }

    pub async fn upload_image(&self, file_path: &str) -> Result<UploadResponse, YhChatError> {
        self.upload("image", file_path).await
    }

    pub async fn upload_video(&self, file_path: &str) -> Result<UploadResponse, YhChatError> {
        self.upload("video", file_path).await
    }

    pub async fn upload_file(&self, file_path: &str) -> Result<UploadResponse, YhChatError> {
        self.upload("file", file_path).await
    }

    pub async fn send_msg_stream(&self, recv_type: &str, recv_id: &str, content_type: &str, content: String) -> Result<StreamSendResponse, YhChatError> {
        let url = format!(
            "{}/bot/send-stream?token={}&recvId={}&recvType={}&contentType={}",
            self.base_url, self.token, recv_id, recv_type, content_type
        );
        
        let content_bytes = content.into_bytes();
        let stream = futures_util::stream::try_unfold(content_bytes, |mut content| async {
            let chunk_size = 1024;
            if content.is_empty() {
                return Ok::<_, std::convert::Infallible>(None);
            }
            let chunk: Vec<u8> = content[..chunk_size.min(content.len())].to_vec();
            content.drain(..chunk.len());
            Ok(Some((chunk, content)))
        });
        
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "text/plain; charset=UTF-8")
            .header("Transfer-Encoding", "chunked")
            .body(reqwest::Body::wrap_stream(stream))
            .send()
            .await?;
        
        let result: StreamSendResponse = response.json().await?;
        if result.code != 1 {
            return Err(YhChatError::ApiError(result.code, result.msg));
        }
        Ok(result)
    }
}