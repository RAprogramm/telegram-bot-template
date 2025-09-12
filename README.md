# Telegram Bot Template

A production-ready template for building Telegram bots with [teloxide](https://github.com/teloxide/teloxide).

## Features

- Long polling for local development.
- Webhook support for production deployments.
- Built-in multilingual greetings (English and Russian).
- Configuration via environment variables.

## Configuration

Set the following environment variables before running the bot:

- `TELOXIDE_TOKEN` – bot token obtained from @BotFather.
- `BOT_LANGUAGE` – `en` (default) or `ru`.
- `RUN_MODE` – `polling` (default) or `webhook`.
- `WEBHOOK_URL` – public HTTPS URL for webhook (required for `webhook` mode).
- `WEBHOOK_ADDR` – address to bind in webhook mode (`0.0.0.0:8443` by default).

## Running

```bash
cargo run
```

The template selects long polling or webhook based on `RUN_MODE`.

## Development

```bash
cargo +nightly fmt --
cargo clippy -D warnings
cargo build --all-targets
cargo test --all
cargo doc --no-deps
cargo audit
```
