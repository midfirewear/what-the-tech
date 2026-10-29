# what-the-tech 🔭

> A Discord bot for tech enthusiasts who want to stay informed — without the noise.

**what-the-tech** monitors curated sources (RSS feeds, public APIs, newsletters) across key tech domains and delivers digests directly into your Discord server. Users subscribe to the topics they care about via slash commands.

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
2. The bot registers their preferences in a local SQLite database
3. Every hour, the bot fetches new items from RSS feeds and APIs for each active topic
4. New items are posted to the subscribed channels as clean embeds

---

## Tech stack

| Layer | Choice | Why |
|-------|--------|-----|
| Language | [Rust](https://www.rust-lang.org/) | Performance, reliability, learning opportunity |
| Discord framework | [poise](https://github.com/serenity-rs/poise) | Built on serenity, great slash command support |
| Database | SQLite via [sqlx](https://github.com/launchbadge/sqlx) | Lightweight, no external server needed |
| RSS parsing | [feed-rs](https://github.com/feed-rs/feed-rs) | Handles RSS, Atom, JSON Feed |
| HTTP client | [reqwest](https://github.com/seanmonstar/reqwest) | Async, ergonomic |
| Scheduling | [tokio](https://tokio.rs/) tasks | Native async runtime, no extra scheduler |

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
- A Discord bot token ([guide](https://discordpy.readthedocs.io/en/stable/discord.html))
- SQLite (bundled via sqlx, no install needed)

### Setup

```bash
git clone https://github.com/midfirewear/what-the-tech
cd what-the-tech
cp .env.example .env
# Fill in DISCORD_TOKEN in .env
cargo run
```

---

## Contributing

This project is built step by step, with readability as a priority. Each feature is introduced in its own commit with a clear message explaining the architectural choice.

PRs welcome — open an issue first to discuss the approach.

---

## License

MIT — see [LICENSE](LICENSE)
