use serde::{Deserialize, Serialize};

use crate::User;

use super::Chat;

/// 消息
/// 文档：https://core.telegram.org/bots/api#message
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// 唯一标识
    pub message_id: u64,

    /// 消息所属的对话
    pub chat: Chat,

    /// 时间戳
    pub date: u64,

    /// 消息发送者
    pub from: Option<User>,

    /// 消息文本
    pub text: Option<String>,
}
