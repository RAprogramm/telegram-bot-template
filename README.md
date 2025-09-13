# Telegram Bot Template

Production‑ready starting point for building Telegram bots with
[teloxide](https://github.com/teloxide/teloxide).

## Quick start

1. **Install Rust** (stable toolchain) and `cargo-generate`:

   ```bash
   curl https://sh.rustup.rs -sSf | sh # if Rust is not installed
   cargo install cargo-generate
   ```

2. **Generate a project** from this template:

   ```bash
   cargo generate --git https://github.com/your-user/telegram-bot-template
   ```

   The generator asks for the bot token, default language and run mode and
   writes them to the newly created `.env` file.

3. **Run the bot**:

   ```bash
   cargo run
   ```

   The runtime selects long polling or webhook based on configuration.

## Configuration

All settings are read from environment variables. The most convenient way is to
create a `.env` file in the project root.

| Variable        | Description                                                              | Default                |
|-----------------|--------------------------------------------------------------------------|------------------------|
| `TELOXIDE_TOKEN`| Bot token issued by [@BotFather](https://t.me/BotFather).                 | – (required)           |
| `BOT_LANGUAGE`  | Interface language: `en` or `ru`.                                        | `en`                   |
| `RUN_MODE`      | Interaction mode: `polling` or `webhook`.                                | `polling`              |
| `WEBHOOK_URL`   | Public HTTPS URL for webhook endpoint (required in `webhook` mode).      | –                      |
| `WEBHOOK_ADDR`  | Local address to bind the webhook listener.                              | `0.0.0.0:8443`         |

## Project layout

```
src/
├── config.rs    # Environment driven configuration loader
├── error.rs     # Unified error type
├── messages.rs  # Localised user‑facing messages
├── run.rs       # Bot dispatcher for polling or webhook modes
├── lib.rs       # Library exports
└── main.rs      # Binary entry point
```

The template exposes a library crate so you can write integration tests or
reuse components from other binaries.

## Development workflow

The project uses stable Rust for builds but relies on the nightly formatter for
consistent styling. Before committing, run:

```bash
cargo +nightly fmt --
cargo clippy -- -D warnings
cargo build --all-targets
cargo test --all
cargo doc --no-deps
cargo audit
```

## Running in webhook mode

Webhook mode is suitable for production deployments where incoming updates are
delivered over HTTPS. To enable it:

1. Provide `RUN_MODE=webhook` and set `WEBHOOK_URL` to the public HTTPS URL
   reachable by Telegram.
2. Optionally adjust `WEBHOOK_ADDR` if the listener should bind to a specific
   interface and port.

The `run` module configures an Axum‑based webhook listener and dispatches
updates to handlers.

## Extending the bot

The default handler replies with a greeting. Add your own logic inside
`run.rs` by matching on incoming `Message` or using Teloxide's command system.
Organise new features into modules to keep concerns separated.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE](LICENSE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)

at your option.

