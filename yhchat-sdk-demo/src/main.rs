use yhchat_sdk_core::*;
use tracing::info;

mod demo {
    use super::*;

    pub async fn test_send_text(client: &YhChatClient, recv_type: &str, recv_id: &str) {
        info!("=== 测试: 发送文本消息 ===");
        match client.send_msg_text(recv_type, recv_id, "测试文本消息 from Rust SDK").await {
            Ok(resp) => info!("✅ 发送成功: {:?}", resp),
            Err(e) => info!("❌ 发送失败: {}", e),
        }
    }

    pub async fn test_send_markdown(client: &YhChatClient, recv_type: &str, recv_id: &str) {
        info!("=== 测试: 发送Markdown消息 ===");
        match client.send_msg_markdown(recv_type, recv_id, "# 标题\n**粗体内容**").await {
            Ok(resp) => info!("✅ 发送成功: {:?}", resp),
            Err(e) => info!("❌ 发送失败: {}", e),
        }
    }

    pub async fn test_send_image(client: &YhChatClient, recv_type: &str, recv_id: &str, image_key: &str) {
        info!("=== 测试: 发送图片消息 ===");
        match client.send_msg_image(recv_type, recv_id, image_key).await {
            Ok(resp) => info!("✅ 发送成功: {:?}", resp),
            Err(e) => info!("❌ 发送失败: {}", e),
        }
    }

    pub async fn test_send_batch(client: &YhChatClient, recv_type: &str, recv_ids: Vec<&str>) {
        info!("=== 测试: 批量发送消息 ===");
        let content = SendContent {
            text: Some("批量测试消息".to_string()),
            image_key: None,
            file_key: None,
            video_key: None,
            buttons: None,
        };
        match client.send_msg_batch(recv_type, recv_ids, "text", content).await {
            Ok(resp) => info!("✅ 批量发送成功: {:?}", resp),
            Err(e) => info!("❌ 批量发送失败: {}", e),
        }
    }

    pub async fn test_edit_msg(client: &YhChatClient, msg_id: &str, recv_type: &str, recv_id: &str) {
        info!("=== 测试: 编辑消息 ===");
        let req = EditMsgRequest {
            msg_id: msg_id.to_string(),
            recv_id: recv_id.to_string(),
            recv_type: recv_type.to_string(),
            content_type: "text".to_string(),
            content: EditContent {
                text: Some("编辑后的内容".to_string()),
                image_url: None,
                file_name: None,
                file_url: None,
                buttons: None,
            },
        };
        match client.edit_msg(req).await {
            Ok(resp) => info!("✅ 编辑成功: {:?}", resp),
            Err(e) => info!("❌ 编辑失败: {}", e),
        }
    }

    pub async fn test_recall_msg(client: &YhChatClient, msg_id: &str, chat_id: &str, chat_type: &str) {
        info!("=== 测试: 撤回消息 ===");
        let req = RecallMsgRequest {
            msg_id: msg_id.to_string(),
            chat_id: chat_id.to_string(),
            chat_type: chat_type.to_string(),
        };
        match client.recall_msg(req).await {
            Ok(resp) => info!("✅ 撤回成功: {:?}", resp),
            Err(e) => info!("❌ 撤回失败: {}", e),
        }
    }

    pub async fn test_get_messages(client: &YhChatClient, chat_id: &str, chat_type: &str) {
        info!("=== 测试: 获取消息列表 ===");
        let req = MessagesRequest {
            chat_id: chat_id.to_string(),
            chat_type: chat_type.to_string(),
            msg_id: None,
            r#before: None,
            after: None,
        };
        match client.messages(req).await {
            Ok(resp) => info!("✅ 获取成功: {:?}", resp),
            Err(e) => info!("❌ 获取失败: {}", e),
        }
    }

    pub async fn test_set_board(client: &YhChatClient, recv_type: &str, recv_id: &str) {
        info!("=== 测试: 设置看板 ===");
        match client.set_board_to_user(recv_id, recv_type, "text", "看板内容").await {
            Ok(resp) => info!("✅ 设置成功: {:?}", resp),
            Err(e) => info!("❌ 设置失败: {}", e),
        }
    }

    pub async fn test_dismiss_board(client: &YhChatClient, recv_type: &str, recv_id: &str) {
        info!("=== 测试: 解散看板 ===");
        match client.dis_board_to_user(recv_id, recv_type).await {
            Ok(resp) => info!("✅ 解散成功: {:?}", resp),
            Err(e) => info!("❌ 解散失败: {}", e),
        }
    }

    pub async fn test_upload_image(client: &YhChatClient, file_path: &str) {
        info!("=== 测试: 上传图片 ===");
        match client.upload_image(file_path).await {
            Ok(resp) => info!("✅ 上传成功: {:?}", resp),
            Err(e) => info!("❌ 上传失败: {}", e),
        }
    }

    pub async fn test_stream_send(client: &YhChatClient, recv_type: &str, recv_id: &str) {
        info!("=== 测试: 流式发送 ===");
        match client.send_msg_stream(recv_type, recv_id, "text", "流式内容...".to_string()).await {
            Ok(resp) => info!("✅ 流式发送成功: {:?}", resp),
            Err(e) => info!("❌ 流式发送失败: {}", e),
        }
    }

    pub fn test_event_parsing() {
        info!("=== 测试: 事件解析 ===");
        
        let sample_event = Event::from_msg_vo(EventMsgVo {
            version: Some("1.0".to_string()),
            header: EventHeader {
                event_id: "test_event_001".to_string(),
                event_time: 1678317600000i64,
                event_type: "message.receive.normal".to_string(),
            },
            event: EventContent {
                time: Some(1678317600000i64),
                chat_id: Some("9140925".to_string()),
                chat_type: Some("user".to_string()),
                group_id: None,
                group_name: None,
                user_id: Some("test_user".to_string()),
                nickname: Some("测试用户".to_string()),
                avatar_url: None,
                setting_json: None,
                sender: Some(EventSender {
                    sender_id: Some("test_sender".to_string()),
                    sender_type: Some("user".to_string()),
                    sender_user_level: None,
                    sender_nickname: Some("测试发送者".to_string()),
                }),
                chat: Some(EventChat {
                    chat_id: Some("9140925".to_string()),
                    chat_type: Some("user".to_string()),
                }),
                message: Some(EventMessage {
                    msg_id: Some("test_msg_001".to_string()),
                    parent_id: None,
                    send_time: Some(1678317600000i64),
                    chat_id: Some("9140925".to_string()),
                    chat_type: Some("user".to_string()),
                    content_type: Some("text".to_string()),
                    content: Some(EventMessageContent {
                        text: Some("测试消息内容".to_string()),
                        image_url: None,
                        image_name: None,
                        file_name: None,
                        file_url: None,
                        file_size: None,
                        etag: None,
                        form_json: None,
                    }),
                    instruction_id: None,
                    instruction_name: None,
                    command_id: None,
                    command_name: None,
                }),
                button: None,
                recv_id: Some("9140925".to_string()),
                recv_type: Some("user".to_string()),
                value: None,
            },
        });
        
        info!("事件解析结果: {:?}", sample_event);
        
        match &sample_event {
            Event::NormalMessage(e) => {
                info!("✅ 普通消息事件解析成功");
                info!("   消息ID: {}", e.msg_id);
                info!("   聊天ID: {}", e.chat_id);
            }
            _ => {
                info!("✅ 其他事件解析成功");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = YhChatClient::new("your_token_here", "https://api.yhchat.com");
    
    demo::test_event_parsing();
    
    demo::test_send_text(&client, "user", "9140925").await;
    demo::test_send_markdown(&client, "user", "9140925").await;
    demo::test_send_batch(&client, "user", vec!["user1", "user2"]).await;
    
    demo::test_get_messages(&client, "9140925", "user").await;
    
    demo::test_set_board(&client, "user", "9140925").await;
    demo::test_dismiss_board(&client, "user", "9140925").await;
    
    demo::test_stream_send(&client, "user", "9140925").await;
    
    info!("所有Demo测试完成");
}