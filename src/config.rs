use std::{env, net::SocketAddr};

use thiserror::Error;
use url::Url;

/// Supported interface language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// English language.
    En,
    /// Russian language.
    Ru,
}

impl Language {
    /// Parses language code from `BOT_LANGUAGE` environment variable.
    ///
    /// # Errors
    /// Returns [`ConfigError::UnsupportedLanguage`] if the language is not supported.
    pub fn from_env() -> Result<Self, ConfigError> {
        match env::var("BOT_LANGUAGE").unwrap_or_else(|_| String::from("en")).as_str() {
            "en" => Ok(Self::En),
            "ru" => Ok(Self::Ru),
            other => Err(ConfigError::UnsupportedLanguage(other.to_owned())),
        }
    }
}

/// Bot run mode.
#[derive(Debug, Clone)]
pub enum RunMode {
    /// Use long polling to receive updates.
    Polling,
    /// Use webhook with binding address and public URL.
    Webhook { addr: SocketAddr, url: Url },
}

/// Application configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// User interface language.
    pub language: Language,
    /// Bot run mode.
    pub run_mode: RunMode,
}

/// Configuration loading errors.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("unsupported language: {0}")]
    UnsupportedLanguage(String),
    #[error("unsupported run mode: {0}")]
    UnsupportedRunMode(String),
    #[error("missing environment variable: {0}")]
    MissingEnvVar(&'static str),
    #[error("invalid address: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("invalid url: {0}")]
    UrlParse(#[from] url::ParseError),
}

impl Config {
    /// Builds configuration from environment variables.
    ///
    /// * `BOT_LANGUAGE` – interface language (`en`, `ru`). Defaults to `en`.
    /// * `RUN_MODE` – `polling` or `webhook`. Defaults to `polling`.
    /// * `WEBHOOK_ADDR` – socket address (`0.0.0.0:8443` by default).
    /// * `WEBHOOK_URL` – public HTTPS URL for webhook (required for `webhook` mode).
    ///
    /// # Errors
    /// Returns [`ConfigError`] if a variable has invalid value or is missing.
    pub fn from_env() -> Result<Self, ConfigError> {
        let language = Language::from_env()?;
        let run_mode =
            match env::var("RUN_MODE").unwrap_or_else(|_| String::from("polling")).as_str() {
                "polling" => RunMode::Polling,
                "webhook" => {
                    let addr: SocketAddr = env::var("WEBHOOK_ADDR")
                        .unwrap_or_else(|_| String::from("0.0.0.0:8443"))
                        .parse()?;
                    let url = env::var("WEBHOOK_URL")
                        .map_err(|_| ConfigError::MissingEnvVar("WEBHOOK_URL"))?;
                    RunMode::Webhook { addr, url: url.parse()? }
                }
                other => return Err(ConfigError::UnsupportedRunMode(other.to_owned())),
            };
        Ok(Self { language, run_mode })
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::{Config, ConfigError, Language, RunMode};

    #[test]
    fn default_config() {
        env::remove_var("BOT_LANGUAGE");
        env::remove_var("RUN_MODE");
        let cfg = Config::from_env().expect("config");
        assert!(matches!(cfg.language, Language::En));
        assert!(matches!(cfg.run_mode, RunMode::Polling));
    }

    #[test]
    fn unsupported_language() {
        env::set_var("BOT_LANGUAGE", "de");
        let err = Config::from_env().unwrap_err();
        match err {
            ConfigError::UnsupportedLanguage(lang) => assert_eq!(lang, "de"),
            other => panic!("unexpected {other:?}"),
        }
        env::remove_var("BOT_LANGUAGE");
    }
}
