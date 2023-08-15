use axum::response::IntoResponse;

#[derive(Debug)]
pub enum Kind {
    Reqwest,
    Serde,
    Config,
}

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn with_cause(kind: Kind, cause: Box<dyn std::error::Error>) -> Self {
        Self {
            kind,
            message: cause.to_string(),
            cause: Some(cause),
        }
    }
    pub fn from_string(kind: Kind, message: String) -> Self {
        Self {
            kind,
            message,
            cause: None,
        }
    }
    pub fn from_str(kind: Kind, msg: &str) -> Self {
        Self::from_string(kind, msg.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::with_cause(Kind::Reqwest, Box::new(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::with_cause(Kind::Serde, Box::new(e))
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Self::with_cause(Kind::Config, Box::new(e))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.message.into_response()
    }
}
