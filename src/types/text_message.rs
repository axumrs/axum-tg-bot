use serde::{Deserialize, Serialize};

use super::ChatID;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum TextMessageChatID {
    Chat(ChatID),
    Username(String),
}

impl From<ChatID> for TextMessageChatID {
    fn from(id: ChatID) -> Self {
        Self::Chat(id)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TextMessageParseMode {
    MarkdownV2,
    HTML,
}

/// 构造用于发送的文本信息
/// 文档：https://core.telegram.org/bots/api#sendmessage
#[derive(Deserialize, Serialize, Debug)]
pub struct TextMessage {
    pub chat_id: TextMessageChatID,
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<TextMessageParseMode>,
    // pub reply_markup:
}

impl TextMessage {
    pub fn new_text(text: &str, chat_id: TextMessageChatID) -> Self {
        Self {
            chat_id,
            text: text.to_string(),
            parse_mode: None,
        }
    }
    pub fn new_markdown(text: &str, chat_id: TextMessageChatID) -> Self {
        Self {
            chat_id,
            text: text.to_string(),
            parse_mode: Some(TextMessageParseMode::MarkdownV2),
        }
    }
}

#[cfg(test)]
mod test {
    use super::TextMessage;

    #[test]
    fn test_serialize_text_msg() {
        let msg = TextMessage {
            // chat_id: super::TextMessageChatID::Chat(1234),
            chat_id: super::TextMessageChatID::Username("foobar".to_string()),
            text: format!("你好，世界"),
            parse_mode: Some(super::TextMessageParseMode::MarkdownV2),
        };
        let txt = serde_json::to_string(&msg).unwrap();
        println!("{}", txt);
    }
}
