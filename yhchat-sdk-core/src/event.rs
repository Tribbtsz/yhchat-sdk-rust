use crate::types::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Event {
    NormalMessage(NormalMessageEvent),
    Instruction(InstructionEvent),
    BotFollowed(BotFollowedEvent),
    BotUnfollowed(BotUnfollowedEvent),
    GroupJoin(GroupJoinEvent),
    GroupLeave(GroupLeaveEvent),
    ButtonClick(ButtonClickEvent),
    BotSetting(BotSettingEvent),
    Unknown(UnknownEvent),
}

#[derive(Debug, Clone)]
pub struct NormalMessageEvent {
    pub msg_id: String,
    pub chat_id: String,
    pub chat_type: String,
    pub content_type: String,
    pub content: EventMessageContent,
    pub sender: EventSender,
    pub raw: EventMsgVo,
}

#[derive(Debug, Clone)]
pub struct InstructionEvent {
    pub msg_id: String,
    pub chat_id: String,
    pub chat_type: String,
    pub content_type: String,
    pub content: EventMessageContent,
    pub sender: EventSender,
    pub command_id: Option<i32>,
    pub command_name: Option<String>,
    pub raw: EventMsgVo,
}

#[derive(Debug, Clone)]
pub struct BotFollowedEvent {
    pub chat_id: String,
    pub chat_type: String,
    pub sender: EventSender,
    pub raw: EventMsgVo,
}

#[derive(Debug, Clone)]
pub struct BotUnfollowedEvent {
    pub chat_id: String,
    pub chat_type: String,
    pub sender: EventSender,
    pub raw: EventMsgVo,
}

#[derive(Debug, Clone)]
pub struct GroupJoinEvent {
    pub chat_id: String,
    pub chat_type: String,
    pub group_id: String,
    pub group_name: String,
    pub sender: EventSender,
    pub raw: EventMsgVo,
}

#[derive(Debug, Clone)]
pub struct GroupLeaveEvent {
    pub chat_id: String,
    pub chat_type: String,
    pub group_id: String,
    pub group_name: String,
    pub sender: EventSender,
    pub raw: EventMsgVo,
}

#[derive(Debug, Clone)]
pub struct ButtonClickEvent {
    pub msg_id: String,
    pub chat_id: String,
    pub chat_type: String,
    pub button_id: String,
    pub button_value: String,
    pub raw: EventMsgVo,
}

#[derive(Debug, Clone)]
pub struct BotSettingEvent {
    pub chat_id: String,
    pub chat_type: String,
    pub raw: EventMsgVo,
}

#[derive(Debug, Clone)]
pub struct UnknownEvent {
    pub event_type: String,
    pub raw: EventMsgVo,
}

impl Event {
    pub fn from_msg_vo(vo: EventMsgVo) -> Self {
        let event_type = vo.header.event_type.clone();
        let sender = vo.event.sender.clone().unwrap_or_default();
        let chat = vo.event.chat.clone().unwrap_or_default();
        let group_id = vo.event.group_id.clone().unwrap_or_default();
        let group_name = vo.event.group_name.clone().unwrap_or_default();
        let button = vo.event.button.clone().unwrap_or_default();
        
        match event_type.as_str() {
            "message.receive.normal" => {
                if let Some(event_msg) = vo.event.message.clone() {
                    let content = event_msg.content.unwrap_or_default();
                    
                    Event::NormalMessage(NormalMessageEvent {
                        msg_id: event_msg.msg_id.unwrap_or_default(),
                        chat_id: chat.chat_id.unwrap_or_default(),
                        chat_type: chat.chat_type.unwrap_or_default(),
                        content_type: event_msg.content_type.unwrap_or_default(),
                        content,
                        sender: sender.clone(),
                        raw: vo.clone(),
                    })
                } else {
                    Event::Unknown(UnknownEvent {
                        event_type,
                        raw: vo,
                    })
                }
            }
            "message.receive.instruction" => {
                if let Some(event_msg) = vo.event.message.clone() {
                    let content = event_msg.content.unwrap_or_default();
                    
                    Event::Instruction(InstructionEvent {
                        msg_id: event_msg.msg_id.unwrap_or_default(),
                        chat_id: chat.chat_id.unwrap_or_default(),
                        chat_type: chat.chat_type.unwrap_or_default(),
                        content_type: event_msg.content_type.unwrap_or_default(),
                        content,
                        sender: sender.clone(),
                        command_id: event_msg.command_id,
                        command_name: event_msg.command_name,
                        raw: vo.clone(),
                    })
                } else {
                    Event::Unknown(UnknownEvent {
                        event_type,
                        raw: vo,
                    })
                }
            }
            "bot.followed" => {
                Event::BotFollowed(BotFollowedEvent {
                    chat_id: chat.chat_id.unwrap_or_default(),
                    chat_type: chat.chat_type.unwrap_or_default(),
                    sender,
                    raw: vo.clone(),
                })
            }
            "bot.unfollowed" => {
                Event::BotUnfollowed(BotUnfollowedEvent {
                    chat_id: chat.chat_id.unwrap_or_default(),
                    chat_type: chat.chat_type.unwrap_or_default(),
                    sender,
                    raw: vo.clone(),
                })
            }
            "group.join" => {
                Event::GroupJoin(GroupJoinEvent {
                    chat_id: chat.chat_id.unwrap_or_default(),
                    chat_type: chat.chat_type.unwrap_or_default(),
                    group_id,
                    group_name,
                    sender,
                    raw: vo.clone(),
                })
            }
            "group.leave" => {
                Event::GroupLeave(GroupLeaveEvent {
                    chat_id: chat.chat_id.unwrap_or_default(),
                    chat_type: chat.chat_type.unwrap_or_default(),
                    group_id,
                    group_name,
                    sender,
                    raw: vo.clone(),
                })
            }
            "button.report.inline" => {
                Event::ButtonClick(ButtonClickEvent {
                    msg_id: button.msg_id.unwrap_or_default(),
                    chat_id: chat.chat_id.unwrap_or_default(),
                    chat_type: chat.chat_type.unwrap_or_default(),
                    button_id: button.button_id.unwrap_or_default(),
                    button_value: button.button_value.unwrap_or_default(),
                    raw: vo.clone(),
                })
            }
            "bot.setting" => {
                Event::BotSetting(BotSettingEvent {
                    chat_id: chat.chat_id.unwrap_or_default(),
                    chat_type: chat.chat_type.unwrap_or_default(),
                    raw: vo.clone(),
                })
            }
            _ => Event::Unknown(UnknownEvent {
                event_type,
                raw: vo,
            }),
        }
    }
}

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_normal_message(&self, event: NormalMessageEvent);
    async fn on_instruction(&self, event: InstructionEvent);
    async fn on_bot_followed(&self, event: BotFollowedEvent);
    async fn on_bot_unfollowed(&self, event: BotUnfollowedEvent);
    async fn on_group_join(&self, event: GroupJoinEvent);
    async fn on_group_leave(&self, event: GroupLeaveEvent);
    async fn on_button_click(&self, event: ButtonClickEvent);
    async fn on_bot_setting(&self, event: BotSettingEvent);
    async fn on_unknown(&self, event: UnknownEvent);
}

pub struct DefaultEventHandler;

#[async_trait::async_trait]
impl EventHandler for DefaultEventHandler {
    async fn on_normal_message(&self, _event: NormalMessageEvent) {}
    async fn on_instruction(&self, _event: InstructionEvent) {}
    async fn on_bot_followed(&self, _event: BotFollowedEvent) {}
    async fn on_bot_unfollowed(&self, _event: BotUnfollowedEvent) {}
    async fn on_group_join(&self, _event: GroupJoinEvent) {}
    async fn on_group_leave(&self, _event: GroupLeaveEvent) {}
    async fn on_button_click(&self, _event: ButtonClickEvent) {}
    async fn on_bot_setting(&self, _event: BotSettingEvent) {}
    async fn on_unknown(&self, _event: UnknownEvent) {}
}

pub struct EventDispatcher {
    handler: Arc<dyn EventHandler>,
}

impl EventDispatcher {
    pub fn new(handler: Arc<dyn EventHandler>) -> Self {
        Self { handler }
    }

    pub async fn dispatch(&self, event: Event) {
        match event {
            Event::NormalMessage(e) => self.handler.on_normal_message(e).await,
            Event::Instruction(e) => self.handler.on_instruction(e).await,
            Event::BotFollowed(e) => self.handler.on_bot_followed(e).await,
            Event::BotUnfollowed(e) => self.handler.on_bot_unfollowed(e).await,
            Event::GroupJoin(e) => self.handler.on_group_join(e).await,
            Event::GroupLeave(e) => self.handler.on_group_leave(e).await,
            Event::ButtonClick(e) => self.handler.on_button_click(e).await,
            Event::BotSetting(e) => self.handler.on_bot_setting(e).await,
            Event::Unknown(e) => self.handler.on_unknown(e).await,
        }
    }

    pub async fn dispatch_raw(&self, vo: EventMsgVo) {
        let event = Event::from_msg_vo(vo);
        self.dispatch(event).await;
    }
}

pub async fn parse_and_dispatch(vo: EventMsgVo, handler: &Arc<dyn EventHandler>) {
    let dispatcher = EventDispatcher::new(handler.clone());
    dispatcher.dispatch_raw(vo).await;
}