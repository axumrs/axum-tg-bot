use serde::{Deserialize, Serialize};

use super::Message;

/// 由 Telegram 传入的更新
/// 文档：https://core.telegram.org/bots/api#update
#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    /// 唯一标识，它从某个正数开始，依次递增。
    /// 在 Webhook 场景中，此标识可以用于：
    /// - 忽略重复的更新
    /// - 对更新进行排序
    pub update_id: u64,

    /// 新传入的消息：文本、图片、贴纸等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    // pub edited_message:Option<Message>,
    // pub channel_post:Option<Message>,
    // pub edited_channel_post:Option<Message>,
}
