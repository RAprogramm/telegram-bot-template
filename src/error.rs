use thiserror::Error;

use crate::config::ConfigError;

/// Errors produced by the bot runtime.
#[derive(Debug, Error)]
pub enum BotError {
    /// Configuration error.
    #[error(transparent)]
    Config(#[from] ConfigError),
    /// Telegram API request error.
    #[error(transparent)]
    Request(#[from] teloxide::RequestError),
}
