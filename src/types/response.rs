use serde::{Deserialize, Serialize};

use super::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub result: Option<Message>,
}
