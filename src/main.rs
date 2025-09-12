use std::sync::Arc;

use telegram_bot_template::{run, BotError, Config};
use teloxide::Bot;

#[tokio::main]
async fn main() -> Result<(), BotError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    dotenvy::dotenv().ok();

    let config = Arc::new(Config::from_env()?);
    let bot = Bot::from_env();

    run(bot, config).await
}
