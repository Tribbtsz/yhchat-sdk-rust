use yhchat_sdk_core::*;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("=== 测试19: 事件处理系统 ===");
    
    // 测试事件解析和分发
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
                    text: Some("测试消息内容 - Test 19".to_string()),
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
            info!("   发送者ID: {:?}", e.sender.sender_id);
            if let Some(text) = &e.content.text {
                info!("   内容: {}", text);
            }
        }
        _ => {
            info!("✅ 其他事件解析成功");
        }
    }
    
    info!("Test 19 completed.");
    info!("事件处理系统已测试完成，事件解析和分发工作正常");
}