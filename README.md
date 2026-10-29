# what-the-tech

A Discord bot for tech watching. It monitors RSS feeds and public APIs across a set of curated topics and posts new items to subscribed channels.

---

## Topics

| Tag | Domain |
|-----|--------|
| `cybersecurity` | Vulnerabilities, exploits, threat intel, security research |
| `ai` | Machine learning, LLMs, research papers, industry news |
| `microelectronics` | Chips, semiconductors, hardware, embedded systems |
| `geopolitics` | Tech policy, international tensions, supply chain |
| `finance` | Markets, crypto, fintech, macro trends |

---

## How it works

1. A user runs `/subscribe ai cybersecurity` in a Discord channel
2. The bot stores their preferences in a local SQLite database
3. Every hour, the bot fetches new items from configured sources for each active topic
4. New items are posted to the subscribed channels as embeds

---

## Tech stack

| Layer | Choice | Why |
|-------|--------|-----|
| Language | [Rust](https://www.rust-lang.org/) | Performance, safety, learning opportunity |
| Discord framework | [poise](https://github.com/serenity-rs/poise) | Built on serenity, clean slash command support |
| Database | SQLite via [sqlx](https://github.com/launchbadge/sqlx) | Lightweight, no external server needed |
| RSS parsing | [feed-rs](https://github.com/feed-rs/feed-rs) | Handles RSS, Atom, JSON Feed |
| HTTP client | [reqwest](https://github.com/seanmonstar/reqwest) | Async, well maintained |
| Scheduling | [tokio](https://tokio.rs/) tasks | Native async runtime |

---

## Project structure

```
what-the-tech/
├── src/
│   ├── main.rs          # Entry point, bot init
│   ├── commands/        # Slash commands (/subscribe, /unsubscribe, /digest)
│   ├── feeds/           # RSS + API fetchers, one module per topic
│   ├── db/              # SQLite schema + queries
│   └── scheduler/       # Periodic fetch + dispatch logic
├── topics.toml          # Source URLs per topic (editable without recompiling)
├── .env.example         # Required environment variables
├── Cargo.toml
└── README.md
```

---

## Getting started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- A Discord bot token ([Discord developer portal](https://discord.com/developers/applications))

### Setup

```bash
git clone https://github.com/midfirewear/what-the-tech
cd what-the-tech
cp .env.example .env
# fill in DISCORD_TOKEN in .env
cargo run
```

---

## Contributing

Each feature is built on its own branch with a clear commit message. Open an issue before sending a PR.

---

## License

MIT
