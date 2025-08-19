use serde::Deserialize;

#[derive(Debug)]
pub enum AbcExportError {
    SerdeJson(serde_json::Error),
    Custom(String),
}

impl From<serde_json::Error> for AbcExportError {
    fn from(value: serde_json::Error) -> Self {
        AbcExportError::SerdeJson(value)
    }
}

impl std::fmt::Display for AbcExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AbcExportError {}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub abc_username: String,
    pub abc_password: String,
    pub admin_email: String,
    pub notifier_email: String,
    pub notifier_passwd: String,
    pub smtp_relay: String,
    pub smtp_port: u16,
}
