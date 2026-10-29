# Context for Claude Code

## Project

Discord bot for tech watching. Posts curated news from RSS feeds and public APIs into subscribed channels. Built in Rust.

Repo: https://github.com/midfirewear/what-the-tech
GitHub account: midfirewear

## Environment

- WSL2 on Windows, username `mdf`, password `mdf`
- Rust installed via rustup, activate with `source ~/.cargo/env` before any cargo command
- OpenSSL dev libs installed (`pkg-config libssl-dev`)
- gh CLI authenticated as `midfirewear` — always prepend `unset GITHUB_TOKEN &&` to gh commands or the old invalid env token takes over
- Discord token is in `.env` (gitignored). Never put it in a markdown file or commit it.

## Git workflow

- `main` : stable, merged manually after user approval
- `dev` : active development branch, always work here
- Never commit directly to main
- Ask user before merging dev into main

## Writing rules

- No em dashes (—) anywhere, in responses or in files
- Short sentences, no AI filler, no bullet summaries of what you just did
- No trailing "next steps" enthusiasm

## Stack

| Layer | Choice |
|-------|--------|
| Language | Rust 1.99 |
| Discord | poise 0.6 (wraps serenity) |
| Database | SQLite via sqlx 0.8 |
| RSS parsing | feed-rs 2.0 |
| HTTP | reqwest 0.12 |
| Scheduling | tokio tasks |
| Config | topics.toml (serde + toml crate) |

## Project structure

```
src/
  main.rs          — bot init, Data struct, startup sequence
  commands/        — one file per slash command
    ping.rs
    subscribe.rs
    unsubscribe.rs
    topics.rs
  db/mod.rs        — all SQLite queries (add/remove/get subscriptions)
  feeds/mod.rs     — topics.toml parsing, TopicsConfig type
  scheduler/       — empty, next milestone
migrations/
  0001_create_subscriptions.sql
topics.toml        — RSS feed URLs per topic, editable without recompiling
.env               — DISCORD_TOKEN, DATABASE_URL, FETCH_INTERVAL_MINUTES
```

## Data model

Single table `subscriptions`:
- `channel_id TEXT` — Discord channel ID
- `topic TEXT` — matches a key from topics.toml
- `added_by TEXT` — Discord user ID who subscribed
- `created_at INTEGER` — unix timestamp
- Primary key: `(channel_id, topic)`

## Data struct (shared state between commands)

```rust
pub struct Data {
    pub db: sqlx::SqlitePool,
    pub topics: Vec<String>,  // loaded from topics.toml at startup
}
```

## Milestones (GitHub)

| # | Name | Status |
|---|------|--------|
| 1 | Foundation | done |
| 2 | Data layer | done |
| 3 | Feed engine | open |
| 4 | Topic system | in progress |
| 5 | Delivery | open |
| 6 | Polish | open |

## Current state

Topic system milestone is in progress (issues #7-#10).
- #7 load topics.toml — done
- #8 /subscribe — done
- #9 /unsubscribe — done
- #10 /topics — done

All committed to `dev`, not yet merged to `main`.

Pending: slash commands registered globally (up to 1h propagation delay on Discord).
For instant testing during dev, switch to guild registration — need the Discord server ID from the user.

## Running the bot

```bash
source ~/.cargo/env
RUST_LOG=info cargo run
```

Bot runs until Ctrl+C. The `.env` file must exist with a valid `DISCORD_TOKEN`.
