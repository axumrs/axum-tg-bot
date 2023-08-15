use serde::{Deserialize, Serialize};
use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use axum_tg_bot::{Bot, Config, Error, Response, Result, TextMessage, Update};

fn get_config() -> Config {
    dotenv::dotenv().ok();
    Config::from_env().unwrap()
}
fn new_bot_with_cfg(cfg: &Config) -> Bot {
    Bot {
        token: cfg.token.clone(),
        chat_id: cfg.chat_id,
    }
}
fn new_bot() -> Bot {
    let cfg = get_config();
    new_bot_with_cfg(&cfg)
}
#[tokio::test]
pub async fn test_set_webhook() {
    let cfg = get_config();
    let bot = new_bot_with_cfg(&cfg);
    let res = bot.set_webhook(&cfg.full_webhook_url()).await.unwrap();
    println!("{}", res);
}

#[tokio::test]
pub async fn test_del_webhook() {
    let bot = new_bot();
    let res = bot.del_webhook().await.unwrap();
    println!("{}", res)
}

#[tokio::test]
pub async fn test_send_text_msg() {
    let bot = new_bot();
    let msg = TextMessage::new_text(
        "👏 哇哦，勤劳的站长又上新了《前置知识：使用 XID 作为分布式ID
》  https://axum.rs/topic/mall/xid",
        bot.chat_id.into(),
    );
    let res = bot.send_text_msg(&msg).await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
pub async fn test_bot_handler() {
    let cfg = get_config();
    let bot = new_bot_with_cfg(&cfg);
    let bot = Arc::new(bot);
    let app = Router::new()
        .route(&cfg.webhook_url, post(bot_handler))
        .with_state(bot);
    axum::Server::bind(&"0.0.0.0:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn bot_handler(
    State(bot): State<Arc<Bot>>,
    Json(update): Json<Update>,
) -> Result<Json<Response>> {
    let msg = update.message.unwrap();
    let msg_text = msg.text.unwrap();
    let chat_id = msg.chat.id;

    let repl_msg = if msg_text == "/get_id" {
        let msg_text = format!("CHAT ID: `{}`", chat_id);
        TextMessage::new_markdown(&msg_text, chat_id.into())
    } else {
        TextMessage::new_text(&msg_text, chat_id.into())
    };

    let res = bot.send_text_msg(&repl_msg).await.map_err(Error::from)?;
    let res = serde_json::from_str(&res).map_err(Error::from)?;
    Ok(Json(res))
}

#[tokio::test]
pub async fn test_github_handler() {
    let bot = new_bot();
    let bot = Arc::new(bot);
    let app = Router::new()
        .route("/github", post(github_handler))
        .with_state(bot);
    axum::Server::bind(&"0.0.0.0:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn github_handler(
    State(bot): State<Arc<Bot>>,
    Json(github): Json<GithubHook>,
) -> Result<String> {
    println!("{:?}", github);
    let msg_text = format!(
        "🎉 天呀，劳模站长更新代码了！\n\n {} @ {} \n\n{}\n\n{}",
        github.head_commit.short_id(),
        github.repository.full_name,
        github.head_commit.message,
        github.head_commit.url,
    );
    let res = bot
        .send_text_msg(&TextMessage::new_text(&msg_text, bot.chat_id.into()))
        .await
        .map_err(Error::from)?;
    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubHook {
    pub repository: GithubHookRepository,
    pub head_commit: GithubHookCommit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubHookRepository {
    pub full_name: String,
    pub html_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GithubHookCommit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
}

impl GithubHookCommit {
    pub fn short_id(&self) -> String {
        let short_id = &self.id[..7];
        short_id.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubHookPusher {
    pub name: String,
}
