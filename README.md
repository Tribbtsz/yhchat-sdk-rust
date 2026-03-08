# YHChat Rust SDK

云湖聊天机器人 Rust SDK，非官方实现。

## 项目结构

```
yhchat-sdk-rust/
├── yhchat-sdk-core/     # 核心库
│   ├── src/
│   │   ├── lib.rs       # 入口
│   │   ├── client.rs    # API客户端
│   │   ├── types.rs     # 数据类型定义
│   │   ├── constants.rs # 常量定义
│   │   └── event.rs     # 事件处理
│   └── Cargo.toml
└── yhchat-sdk-demo/     # 示例项目
    ├── src/main.rs
    └── Cargo.toml
```

## 已测试通过的功能

| 功能 | 状态 |
|------|------|
| 发送文本消息 | ✅ |
| 发送Markdown消息 | ✅ |
| 发送HTML消息 | ✅ |
| 发送图片消息 (通过imageKey) | ✅ |
| 发送文件消息 (通过fileKey) | ✅ |
| 发送视频消息 (通过videoKey) | ✅ |
| 批量发送消息 | ✅ |
| 编辑消息 | ✅ |
| 撤回消息 | ✅ |
| 获取消息列表 | ✅ |
| 设置看板 (单用户/群) | ✅ |
| 取消看板 (单用户/群) | ✅ |
| 上传图片 | ✅ |
| 流式发送消息 (send_msg_stream) | ✅ |
| 上传视频 | ✅ |
| 上传文件 | ✅ |
| 设置全局看板 (board-all) | ✅ |
| 取消全局看板 (board-all-dismiss) | ✅ |
| 事件处理系统 (EventHandler) | ✅ |
| 指令消息处理 (commandId/commandName) | ✅ |

## 事件处理

SDK 提供了完整的事件处理系统，支持以下事件类型：

- `message.receive.normal` - 普通消息
- `message.receive.instruction` - 指令消息（含commandId/commandName）
- `bot.followed` - 机器人关注
- `bot.unfollowed` - 机器人取消关注
- `group.join` - 加入群聊
- `group.leave` - 离开群聊
- `button.report.inline` - 按钮点击
- `bot.setting` - 机器人设置

事件处理系统包括：
- `Event` 枚举 - 所有事件类型
- `EventHandler` trait - 异步事件处理接口
- `EventDispatcher` - 事件分发器
- `DefaultEventHandler` - 默认空实现

使用示例：

```rust
use yhchat_sdk_core::*;
use std::sync::Arc;

struct MyEventHandler {
    _client: YhChatClient,
}

#[async_trait::async_trait]
impl EventHandler for MyEventHandler {
    async fn on_normal_message(&self, event: NormalMessageEvent) {
        println!("收到普通消息: {:?}", event.content.text);
    }
    
    async fn on_instruction(&self, event: InstructionEvent) {
        println!("收到指令消息: commandId={:?}, commandName={:?}", 
                 event.command_id, event.command_name);
    }
    
    // ... 其他事件处理方法
}

#[tokio::main]
async fn main() {
    let token = "your_token";
    let client = YhChatClient::new(token);
    let handler = Arc::new(MyEventHandler { _client: client });
    
    // 使用事件分发器
    let dispatcher = EventDispatcher::new(handler.clone());
    // dispatcher.dispatch_raw(event_msg_vo).await;
}
```

## 待实现/完善

所有核心功能已完成实现并通过测试。

## 快速开始

```rust
use yhchat_sdk_core::YhChatClient;

#[tokio::main]
async fn main() {
    let token = "your_token";
    let client = YhChatClient::new(token);
    
    // 发送文本消息
    match client.send_msg_text("user", "user_id", "Hello!").await {
        Ok(resp) => println!("Success: {:?}", resp),
        Err(e) => println!("Error: {}", e),
    }
}
```

## 配置

在demo中修改以下配置：

```rust
let token = "f34b55307b394117ba110883bdf2a260"; // 机器人Token
let user_id = "9140925";  // 用户ID
let group_id = "741947930"; // 群ID
let bot_id = "92416686"; // 机器人ID
```

## API基础URL

```
https://chat-go.jwzhd.com/open-apis/v1
```

## 测试用的回调地址

```
https://f545-2406-da18-10-2c00-bb5f-fa68-6699-1b4d.ngrok-free.app/yhchat/event/msg
```

## 参考

- 官方文档：https://www.yhchat.com/document/1-3
- Java SDK源码：`../yhchat-sdk-core/`
- Java Demo源码：`../yhchat-sdk-demo/`