use serde::Serialize;

use crate::ChatID;

use super::TextMessage;

pub struct Bot {
    pub token: String,
    pub chat_id: ChatID,
}

impl Bot {
    /// 调用 API
    async fn invoke<T: Serialize>(
        &self,
        data: &T,
        method: &str,
    ) -> std::result::Result<String, reqwest::Error> {
        let api_addr = format!("https://api.telegram.org/bot{}/{}", &self.token, method);
        reqwest::Client::new()
            .post(&api_addr)
            .form(data)
            .send()
            .await?
            .text()
            .await
    }

    /// 发送文本信息
    pub async fn send_text_msg(
        &self,
        msg: &TextMessage,
    ) -> std::result::Result<String, reqwest::Error> {
        self.invoke(msg, "sendMessage").await
    }

    async fn invoke_webhook(
        &self,
        method: &str,
        param: Option<String>,
    ) -> std::result::Result<String, reqwest::Error> {
        let api_addr = match param {
            Some(p) => format!(
                "https://api.telegram.org/bot{}/{}?{}",
                &self.token, method, p
            ),
            None => format!("https://api.telegram.org/bot{}/{}", &self.token, method),
        };
        reqwest::Client::new()
            .get(api_addr)
            .send()
            .await?
            .text()
            .await
    }

    pub async fn set_webhook(&self, url: &str) -> std::result::Result<String, reqwest::Error> {
        self.invoke_webhook("setWebhook", Some(format!("url={}", url)))
            .await
    }
    pub async fn del_webhook(&self) -> std::result::Result<String, reqwest::Error> {
        self.invoke_webhook("deleteWebhook", None).await
    }
}
