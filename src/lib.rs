//! Library for Telegram bot template.

pub mod config;
pub mod error;
pub mod messages;
pub mod run;

pub use config::{Config, Language, RunMode};
pub use error::BotError;
pub use run::run;
