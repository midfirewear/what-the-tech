mod commands;
mod db;
mod feeds;
mod scheduler;

use poise::serenity_prelude as serenity;

pub struct Data {
    pub db: sqlx::SqlitePool,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN in .env");
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://what-the-tech.db".to_string());
    let intents = serenity::GatewayIntents::non_privileged();

    let pool = db::create_pool(&database_url)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("database migration failed");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping::ping()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            let pool = pool.clone();
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { db: pool })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("failed to create Discord client");

    client.start().await.expect("bot crashed");
}
