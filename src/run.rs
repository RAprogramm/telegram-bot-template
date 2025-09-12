use std::sync::Arc;

use teloxide::{
    repl, repl_with_listener, requests::Requester, types::Message, update_listeners::webhooks, Bot,
};

use crate::{
    config::{Config, RunMode},
    error::BotError,
    messages,
};

/// Runs the bot according to the provided configuration.
///
/// # Errors
/// Returns [`BotError`] if webhook setup fails.
pub async fn run(bot: Bot, config: Arc<Config>) -> Result<(), BotError> {
    match &config.run_mode {
        RunMode::Polling => {
            repl(bot, move |bot: Bot, msg: Message| {
                let cfg = Arc::clone(&config);
                async move {
                    bot.send_message(msg.chat.id, messages::greeting(cfg.language)).await?;
                    Ok(())
                }
            })
            .await;
        }
        RunMode::Webhook { addr, url } => {
            let listener =
                webhooks::axum(bot.clone(), webhooks::Options::new(*addr, url.clone())).await?;
            repl_with_listener(
                bot,
                move |bot: Bot, msg: Message| {
                    let cfg = Arc::clone(&config);
                    async move {
                        bot.send_message(msg.chat.id, messages::greeting(cfg.language)).await?;
                        Ok(())
                    }
                },
                listener,
            )
            .await;
        }
    }
    Ok(())
}
