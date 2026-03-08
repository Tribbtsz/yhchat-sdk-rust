pub mod recv_type {
    pub const USER: &str = "user";
    pub const GROUP: &str = "group";
}

pub mod content_type {
    pub const TEXT: &str = "text";
    pub const IMAGE: &str = "image";
    pub const VIDEO: &str = "video";
    pub const FILE: &str = "file";
    pub const MARKDOWN: &str = "markdown";
    pub const HTML: &str = "html";
    pub const FORM: &str = "form";
}

pub mod chat_type {
    pub const BOT: &str = "bot";
    pub const GROUP: &str = "group";
}

pub mod sender_level {
    pub const OWNER: &str = "owner";
    pub const ADMINISTRATOR: &str = "administrator";
    pub const MEMBER: &str = "member";
    pub const UNKNOWN: &str = "unknown";
}

pub mod button_action_type {
    pub const JUMP_URL: i32 = 1;
    pub const COPY: i32 = 2;
    pub const REPORT: i32 = 3;
}

pub mod event_type {
    pub const MESSAGE_RECEIVE_NORMAL: &str = "message.receive.normal";
    pub const MESSAGE_RECEIVE_INSTRUCTION: &str = "message.receive.instruction";
    pub const BOT_FOLLOWED: &str = "bot.followed";
    pub const BOT_UNFOLLOWED: &str = "bot.unfollowed";
    pub const GROUP_JOIN: &str = "group.join";
    pub const GROUP_LEAVE: &str = "group.leave";
    pub const BUTTON_REPORT_INLINE: &str = "button.report.inline";
    pub const BOT_SETTING: &str = "bot.setting";
}