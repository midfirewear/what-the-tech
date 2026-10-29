use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);
    SqlitePool::connect_with(options).await
}

pub async fn add_subscription(
    pool: &SqlitePool,
    channel_id: &str,
    topic: &str,
    added_by: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR IGNORE INTO subscriptions (channel_id, topic, added_by) VALUES (?, ?, ?)",
    )
    .bind(channel_id)
    .bind(topic)
    .bind(added_by)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_subscription(
    pool: &SqlitePool,
    channel_id: &str,
    topic: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM subscriptions WHERE channel_id = ? AND topic = ?")
        .bind(channel_id)
        .bind(topic)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_channels_for_topic(
    pool: &SqlitePool,
    topic: &str,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String,)>(
        "SELECT channel_id FROM subscriptions WHERE topic = ?",
    )
    .bind(topic)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

pub async fn get_topics_for_channel(
    pool: &SqlitePool,
    channel_id: &str,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String,)>(
        "SELECT topic FROM subscriptions WHERE channel_id = ?",
    )
    .bind(channel_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(topic,)| topic).collect())
}
