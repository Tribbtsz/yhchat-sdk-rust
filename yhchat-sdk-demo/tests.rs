use yhchat_sdk_core::YhChatClient;

pub async fn test_1_send_text(client: &YhChatClient) {
    println!("=== 测试1: 发送文本消息 ===");
    
    match client.send_msg_text("user", "9140925", "测试文本消息 from Rust SDK - Test 1").await {
        Ok(resp) => println!("✅ 发送成功: {:?}", resp),
        Err(e) => println!("❌ 发送失败: {}", e),
    }
}