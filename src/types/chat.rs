use serde::{Deserialize, Serialize};

/// 对话ID
pub type ChatID = i64;

/// 聊天对话
/// 文档：https://core.telegram.org/bots/api#chat
#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    /// 唯一标识
    pub id: ChatID,

    /// 聊天类型
    #[serde(rename = "type")]
    pub chat_type: String,

    /// 标题。适用于群组、超级群组、频道
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// 用户名（如果设置了用户名）。用于私人、超级群组和频道
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// 私人聊天中，对方的名字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// 私人聊天中，对方的姓氏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// #[serde(rename_all = "lowercase")]
// pub enum ChatType {
//     /// 私人
//     Private,
//     /// 群组
//     Group,
//     /// 超级群组
//     Supergroup,
//     /// 频道
//     Channel,
// }
