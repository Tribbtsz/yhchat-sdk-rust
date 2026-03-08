use yhchat_sdk_core::*;
use std::sync::Arc;

fn get_test_client() -> YhChatClient {
    YhChatClient::new("test_token", "https://api.yhchat.com")
}

fn create_test_event(event_type: &str) -> EventMsgVo {
    EventMsgVo {
        version: Some("1.0".to_string()),
        header: EventHeader {
            event_id: "test_event_001".to_string(),
            event_time: 1678317600000i64,
            event_type: event_type.to_string(),
        },
        event: EventContent {
            time: Some(1678317600000i64),
            chat_id: Some("9140925".to_string()),
            chat_type: Some("user".to_string()),
            group_id: Some("group_001".to_string()),
            group_name: Some("测试群组".to_string()),
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
                command_id: Some(1),
                command_name: Some("test_command".to_string()),
            }),
            button: Some(EventButton {
                msg_id: Some("test_msg_001".to_string()),
                button_id: Some("btn_001".to_string()),
                button_value: Some("click_value".to_string()),
            }),
            recv_id: Some("9140925".to_string()),
            recv_type: Some("user".to_string()),
            value: None,
        },
    }
}

mod test_01_client_creation {
    use super::*;
    
    #[tokio::test]
    async fn test_client_new() {
        let client = YhChatClient::new("test_token", "https://api.yhchat.com");
        assert_eq!(client.get_token(), "test_token");
        println!("✅ 测试1: 客户端创建成功");
    }
}

mod test_02_send_text {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_text_request() {
        let req = SendMsgRequest::text("user", "9140925", "测试文本消息");
        assert_eq!(req.recv_type, "user");
        assert_eq!(req.recv_id, "9140925");
        assert_eq!(req.content_type, "text");
        assert_eq!(req.content.text, Some("测试文本消息".to_string()));
        println!("✅ 测试2: 文本消息请求构建成功");
    }
}

mod test_03_send_markdown {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_markdown_request() {
        let req = SendMsgRequest::markdown("user", "9140925", "# 标题\n**粗体**");
        assert_eq!(req.content_type, "markdown");
        assert_eq!(req.content.text, Some("# 标题\n**粗体**".to_string()));
        println!("✅ 测试3: Markdown消息请求构建成功");
    }
}

mod test_04_send_image {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_image_request() {
        let req = SendMsgRequest::image("user", "9140925", "image_key_123");
        assert_eq!(req.content_type, "image");
        assert_eq!(req.content.image_key, Some("image_key_123".to_string()));
        println!("✅ 测试4: 图片消息请求构建成功");
    }
}

mod test_05_send_file {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_file_request() {
        let req = SendMsgRequest::file("user", "9140925", "file_key_456");
        assert_eq!(req.content_type, "file");
        assert_eq!(req.content.file_key, Some("file_key_456".to_string()));
        println!("✅ 测试5: 文件消息请求构建成功");
    }
}

mod test_06_send_video {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_video_request() {
        let req = SendMsgRequest::video("user", "9140925", "video_key_789");
        assert_eq!(req.content_type, "video");
        assert_eq!(req.content.video_key, Some("video_key_789".to_string()));
        println!("✅ 测试6: 视频消息请求构建成功");
    }
}

mod test_07_send_html {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_html_request() {
        let req = SendMsgRequest::html("user", "9140925", "<b>HTML内容</b>");
        assert_eq!(req.content_type, "html");
        assert_eq!(req.content.text, Some("<b>HTML内容</b>".to_string()));
        println!("✅ 测试7: HTML消息请求构建成功");
    }
}

mod test_08_send_with_buttons {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_with_buttons() {
        let mut req = SendMsgRequest::text("user", "9140925", "带按钮的消息");
        req.add_button("点击跳转", 1, Some("https://example.com"), None);
        req.add_button("复制内容", 2, None, Some("copy_value"));
        
        assert!(req.content.buttons.is_some());
        let buttons = req.content.buttons.unwrap();
        assert_eq!(buttons.len(), 2);
        assert_eq!(buttons[0].action_type, 1);
        assert_eq!(buttons[1].action_type, 2);
        println!("✅ 测试8: 带按钮消息请求构建成功");
    }
}

mod test_09_send_batch {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_batch_request() {
        let content = SendContent {
            text: Some("批量消息".to_string()),
            image_key: None,
            file_key: None,
            video_key: None,
            buttons: None,
        };
        let req = SendMsgBatchRequest {
            recv_ids: vec!["user1".to_string(), "user2".to_string()],
            recv_type: "user".to_string(),
            content_type: "text".to_string(),
            content,
        };
        
        assert_eq!(req.recv_ids.len(), 2);
        println!("✅ 测试9: 批量消息请求构建成功");
    }
}

mod test_10_edit_message {
    use super::*;
    
    #[tokio::test]
    async fn test_edit_msg_request() {
        let req = EditMsgRequest {
            msg_id: "msg_001".to_string(),
            recv_id: "9140925".to_string(),
            recv_type: "user".to_string(),
            content_type: "text".to_string(),
            content: EditContent {
                text: Some("编辑后的内容".to_string()),
                image_url: None,
                file_name: None,
                file_url: None,
                buttons: None,
            },
        };
        
        assert_eq!(req.msg_id, "msg_001");
        assert_eq!(req.content.text, Some("编辑后的内容".to_string()));
        println!("✅ 测试10: 编辑消息请求构建成功");
    }
}

mod test_11_recall_message {
    use super::*;
    
    #[tokio::test]
    async fn test_recall_msg_request() {
        let req = RecallMsgRequest {
            msg_id: "msg_001".to_string(),
            chat_id: "9140925".to_string(),
            chat_type: "user".to_string(),
        };
        
        assert_eq!(req.msg_id, "msg_001");
        println!("✅ 测试11: 撤回消息请求构建成功");
    }
}

mod test_12_get_messages {
    use super::*;
    
    #[tokio::test]
    async fn test_messages_request() {
        let req = MessagesRequest {
            chat_id: "9140925".to_string(),
            chat_type: "user".to_string(),
            msg_id: Some("msg_001".to_string()),
            r#before: Some("10".to_string()),
            after: None,
        };
        
        assert_eq!(req.chat_id, "9140925");
        assert!(req.msg_id.is_some());
        println!("✅ 测试12: 获取消息列表请求构建成功");
    }
}

mod test_13_set_board {
    use super::*;
    
    #[tokio::test]
    async fn test_set_board_request() {
        let req = SetBoardRequest {
            recv_id: Some("9140925".to_string()),
            recv_type: Some("user".to_string()),
            content_type: "text".to_string(),
            content: "看板内容".to_string(),
        };
        
        assert_eq!(req.content, "看板内容");
        println!("✅ 测试13: 设置看板请求构建成功");
    }
}

mod test_14_set_board_all {
    use super::*;
    
    #[tokio::test]
    async fn test_set_board_all_request() {
        let req = SetBoardRequest {
            recv_id: None,
            recv_type: None,
            content_type: "text".to_string(),
            content: "全员看板内容".to_string(),
        };
        
        assert!(req.recv_id.is_none());
        println!("✅ 测试14: 全员看板请求构建成功");
    }
}

mod test_15_dismiss_board {
    use super::*;
    
    #[tokio::test]
    async fn test_dismiss_board_request() {
        let req = DisBoardRequest {
            recv_id: Some("9140925".to_string()),
            recv_type: Some("user".to_string()),
        };
        
        assert_eq!(req.recv_id, Some("9140925".to_string()));
        println!("✅ 测试15: 解散看板请求构建成功");
    }
}

mod test_16_upload_request {
    use super::*;
    
    #[tokio::test]
    async fn test_upload_request() {
        let req = UploadRequest {
            file_path: "/path/to/file.png".to_string(),
        };
        
        assert_eq!(req.file_path, "/path/to/file.png");
        println!("✅ 测试16: 上传请求构建成功");
    }
}

mod test_17_stream_send_request {
    use super::*;
    
    #[tokio::test]
    async fn test_stream_send_request() {
        let req = StreamSendRequest {
            recv_id: "9140925".to_string(),
            recv_type: "user".to_string(),
            content_type: "text".to_string(),
        };
        
        assert_eq!(req.recv_id, "9140925");
        println!("✅ 测试17: 流式发送请求构建成功");
    }
}

mod test_18_constants {
    use super::*;
    
    #[tokio::test]
    async fn test_constants() {
        assert_eq!(recv_type::USER, "user");
        assert_eq!(recv_type::GROUP, "group");
        assert_eq!(content_type::TEXT, "text");
        assert_eq!(content_type::IMAGE, "image");
        assert_eq!(content_type::VIDEO, "video");
        assert_eq!(content_type::FILE, "file");
        assert_eq!(content_type::MARKDOWN, "markdown");
        assert_eq!(content_type::HTML, "html");
        assert_eq!(button_action_type::JUMP_URL, 1);
        assert_eq!(button_action_type::COPY, 2);
        assert_eq!(button_action_type::REPORT, 3);
        println!("✅ 测试18: 常量定义验证成功");
    }
}

mod test_19_event_normal_message {
    use super::*;
    
    #[tokio::test]
    async fn test_event_normal_message() {
        let vo = create_test_event(event_type::MESSAGE_RECEIVE_NORMAL);
        let event = Event::from_msg_vo(vo);
        
        match event {
            Event::NormalMessage(e) => {
                assert_eq!(e.msg_id, "test_msg_001");
                assert_eq!(e.chat_id, "9140925");
                assert_eq!(e.content_type, "text");
                println!("✅ 测试19: 普通消息事件解析成功");
            }
            _ => panic!("事件类型不匹配"),
        }
    }
}

mod test_20_event_instruction {
    use super::*;
    
    #[tokio::test]
    async fn test_event_instruction() {
        let vo = create_test_event(event_type::MESSAGE_RECEIVE_INSTRUCTION);
        let event = Event::from_msg_vo(vo);
        
        match event {
            Event::Instruction(e) => {
                assert_eq!(e.msg_id, "test_msg_001");
                assert!(e.command_id.is_some());
                println!("✅ 测试20: 指令消息事件解析成功");
            }
            _ => panic!("事件类型不匹配"),
        }
    }
}

mod test_21_event_bot_followed {
    use super::*;
    
    #[tokio::test]
    async fn test_event_bot_followed() {
        let vo = create_test_event(event_type::BOT_FOLLOWED);
        let event = Event::from_msg_vo(vo);
        
        match event {
            Event::BotFollowed(e) => {
                assert_eq!(e.chat_id, "9140925");
                println!("✅ 测试21: Bot被关注事件解析成功");
            }
            _ => panic!("事件类型不匹配"),
        }
    }
}

mod test_22_event_bot_unfollowed {
    use super::*;
    
    #[tokio::test]
    async fn test_event_bot_unfollowed() {
        let vo = create_test_event(event_type::BOT_UNFOLLOWED);
        let event = Event::from_msg_vo(vo);
        
        match event {
            Event::BotUnfollowed(e) => {
                assert_eq!(e.chat_id, "9140925");
                println!("✅ 测试22: Bot被取消关注事件解析成功");
            }
            _ => panic!("事件类型不匹配"),
        }
    }
}

mod test_23_event_group_join {
    use super::*;
    
    #[tokio::test]
    async fn test_event_group_join() {
        let vo = create_test_event(event_type::GROUP_JOIN);
        let event = Event::from_msg_vo(vo);
        
        match event {
            Event::GroupJoin(e) => {
                assert_eq!(e.group_id, "group_001");
                assert_eq!(e.group_name, "测试群组");
                println!("✅ 测试23: 加入群组事件解析成功");
            }
            _ => panic!("事件类型不匹配"),
        }
    }
}

mod test_24_event_group_leave {
    use super::*;
    
    #[tokio::test]
    async fn test_event_group_leave() {
        let vo = create_test_event(event_type::GROUP_LEAVE);
        let event = Event::from_msg_vo(vo);
        
        match event {
            Event::GroupLeave(e) => {
                assert_eq!(e.group_id, "group_001");
                println!("✅ 测试24: 离开群组事件解析成功");
            }
            _ => panic!("事件类型不匹配"),
        }
    }
}

mod test_25_event_button_click {
    use super::*;
    
    #[tokio::test]
    async fn test_event_button_click() {
        let vo = create_test_event(event_type::BUTTON_REPORT_INLINE);
        let event = Event::from_msg_vo(vo);
        
        match event {
            Event::ButtonClick(e) => {
                assert_eq!(e.button_id, "btn_001");
                assert_eq!(e.button_value, "click_value");
                println!("✅ 测试25: 按钮点击事件解析成功");
            }
            _ => panic!("事件类型不匹配"),
        }
    }
}

mod test_26_event_bot_setting {
    use super::*;
    
    #[tokio::test]
    async fn test_event_bot_setting() {
        let vo = create_test_event(event_type::BOT_SETTING);
        let event = Event::from_msg_vo(vo);
        
        match event {
            Event::BotSetting(e) => {
                assert_eq!(e.chat_id, "9140925");
                println!("✅ 测试26: Bot设置事件解析成功");
            }
            _ => panic!("事件类型不匹配"),
        }
    }
}

mod test_27_event_unknown {
    use super::*;
    
    #[tokio::test]
    async fn test_event_unknown() {
        let vo = create_test_event("unknown.event.type");
        let event = Event::from_msg_vo(vo);
        
        match event {
            Event::Unknown(e) => {
                assert_eq!(e.event_type, "unknown.event.type");
                println!("✅ 测试27: 未知事件解析成功");
            }
            _ => panic!("事件类型不匹配"),
        }
    }
}

mod test_28_event_dispatcher {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct TestHandler {
        call_count: Arc<AtomicUsize>,
    }
    
    #[async_trait::async_trait]
    impl EventHandler for TestHandler {
        async fn on_normal_message(&self, _event: NormalMessageEvent) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
        async fn on_instruction(&self, _event: InstructionEvent) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
        async fn on_bot_followed(&self, _event: BotFollowedEvent) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
        async fn on_bot_unfollowed(&self, _event: BotUnfollowedEvent) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
        async fn on_group_join(&self, _event: GroupJoinEvent) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
        async fn on_group_leave(&self, _event: GroupLeaveEvent) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
        async fn on_button_click(&self, _event: ButtonClickEvent) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
        async fn on_bot_setting(&self, _event: BotSettingEvent) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
        async fn on_unknown(&self, _event: UnknownEvent) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
    }
    
    #[tokio::test]
    async fn test_event_dispatcher() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let handler = Arc::new(TestHandler {
            call_count: call_count.clone(),
        });
        
        let dispatcher = EventDispatcher::new(handler);
        let vo = create_test_event(event_type::MESSAGE_RECEIVE_NORMAL);
        dispatcher.dispatch_raw(vo).await;
        
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
        println!("✅ 测试28: 事件分发器测试成功");
    }
}

mod test_29_default_event_handler {
    use super::*;
    
    #[tokio::test]
    async fn test_default_event_handler() {
        let handler = Arc::new(DefaultEventHandler);
        let dispatcher = EventDispatcher::new(handler);
        
        let vo = create_test_event(event_type::MESSAGE_RECEIVE_NORMAL);
        dispatcher.dispatch_raw(vo).await;
        
        println!("✅ 测试29: 默认事件处理器测试成功");
    }
}

mod test_30_parse_and_dispatch {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct CountHandler {
        count: Arc<AtomicUsize>,
    }
    
    #[async_trait::async_trait]
    impl EventHandler for CountHandler {
        async fn on_normal_message(&self, _event: NormalMessageEvent) {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
        async fn on_instruction(&self, _event: InstructionEvent) {}
        async fn on_bot_followed(&self, _event: BotFollowedEvent) {}
        async fn on_bot_unfollowed(&self, _event: BotUnfollowedEvent) {}
        async fn on_group_join(&self, _event: GroupJoinEvent) {}
        async fn on_group_leave(&self, _event: GroupLeaveEvent) {}
        async fn on_button_click(&self, _event: ButtonClickEvent) {}
        async fn on_bot_setting(&self, _event: BotSettingEvent) {}
        async fn on_unknown(&self, _event: UnknownEvent) {}
    }
    
    #[tokio::test]
    async fn test_parse_and_dispatch() {
        let count = Arc::new(AtomicUsize::new(0));
        let handler = Arc::new(CountHandler { count: count.clone() });
        
        let vo = create_test_event(event_type::MESSAGE_RECEIVE_NORMAL);
        parse_and_dispatch(vo, &handler).await;
        
        assert_eq!(count.load(Ordering::SeqCst), 1);
        println!("✅ 测试30: parse_and_dispatch函数测试成功");
    }
}

mod test_31_serialization {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_request_serialization() {
        let req = SendMsgRequest::text("user", "9140925", "测试");
        let json = serde_json::to_string(&req).unwrap();
        
        assert!(json.contains("recvId"));
        assert!(json.contains("recvType"));
        assert!(json.contains("contentType"));
        println!("✅ 测试31: 序列化测试成功");
    }
}

mod test_32_deserialization {
    use super::*;
    
    #[tokio::test]
    async fn test_send_msg_response_deserialization() {
        let json = r#"{"code":1,"msg":"success","data":{"messageInfo":{"msgId":"msg_001","recvId":"9140925","recvType":"user"}}}"#;
        let resp: SendMsgResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(resp.code, 1);
        assert!(resp.data.is_some());
        println!("✅ 测试32: 反序列化测试成功");
    }
}

mod test_33_error_types {
    use super::*;
    
    #[tokio::test]
    async fn test_error_types() {
        let api_error = YhChatError::ApiError(100, "错误消息".to_string());
        let error_msg = format!("{}", api_error);
        assert!(error_msg.contains("100"));
        println!("✅ 测试33: 错误类型测试成功");
    }
}

mod test_34_event_sender_default {
    use super::*;
    
    #[tokio::test]
    async fn test_event_sender_default() {
        let sender = EventSender::default();
        assert!(sender.sender_id.is_none());
        assert!(sender.sender_type.is_none());
        println!("✅ 测试34: EventSender默认值测试成功");
    }
}

mod test_35_event_chat_default {
    use super::*;
    
    #[tokio::test]
    async fn test_event_chat_default() {
        let chat = EventChat::default();
        assert!(chat.chat_id.is_none());
        assert!(chat.chat_type.is_none());
        println!("✅ 测试35: EventChat默认值测试成功");
    }
}

mod test_36_event_button_default {
    use super::*;
    
    #[tokio::test]
    async fn test_event_button_default() {
        let button = EventButton::default();
        assert!(button.msg_id.is_none());
        assert!(button.button_id.is_none());
        println!("✅ 测试36: EventButton默认值测试成功");
    }
}

mod test_37_event_message_content_default {
    use super::*;
    
    #[tokio::test]
    async fn test_event_message_content_default() {
        let content = EventMessageContent::default();
        assert!(content.text.is_none());
        assert!(content.image_url.is_none());
        println!("✅ 测试37: EventMessageContent默认值测试成功");
    }
}

mod test_38_api_result {
    use super::*;
    
    #[tokio::test]
    async fn test_api_result() {
        let json = r#"{"code":1,"msg":"success","data":{"test":"value"}}"#;
        let result: ApiResult<serde_json::Value> = serde_json::from_str(json).unwrap();
        
        assert_eq!(result.code, 1);
        assert_eq!(result.msg, "success");
        println!("✅ 测试38: ApiResult测试成功");
    }
}

mod test_39_multiple_buttons {
    use super::*;
    
    #[tokio::test]
    async fn test_multiple_buttons() {
        let mut req = SendMsgRequest::text("user", "9140925", "多按钮测试");
        req.add_button("按钮1", 1, Some("https://a.com"), None);
        req.add_button("按钮2", 2, None, Some("value2"));
        req.add_button("按钮3", 3, None, Some("value3"));
        
        let buttons = req.content.buttons.as_ref().unwrap();
        assert_eq!(buttons.len(), 3);
        println!("✅ 测试39: 多按钮测试成功");
    }
}

mod test_40_upload_response {
    use super::*;
    
    #[tokio::test]
    async fn test_upload_response() {
        let json = r#"{"code":1,"msg":"success","data":{"imageKey":"img_001","videoKey":null,"fileKey":null}}"#;
        let resp: UploadResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(resp.code, 1);
        assert!(resp.data.is_some());
        let data = resp.data.unwrap();
        assert_eq!(data.image_key, Some("img_001".to_string()));
        println!("✅ 测试40: UploadResponse测试成功");
    }
}
