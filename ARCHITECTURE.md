# Architecture

This template separates configuration, message localisation and runtime logic:

- `config` loads settings from environment variables and validates them.
- `messages` provides localised strings.
- `run` starts the bot either with polling or webhooks depending on configuration.

## Alternatives Considered

- **Hard‑coded polling** – rejected to allow production deployments on webhooks.
- **External i18n crate** – overkill for a small template; simple enum-based approach is used instead.

## Failure Modes

- Missing or invalid environment variables: the application exits with an error.
- Network failures when calling Telegram API: `teloxide` propagates `RequestError`.
- Webhook misconfiguration: startup fails while initialising the listener.
