use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct TopicConfig {
    pub feeds: Vec<String>,
}

pub type TopicsConfig = HashMap<String, TopicConfig>;

pub fn load_topics(path: &str) -> Result<TopicsConfig, Box<dyn std::error::Error + Send + Sync>> {
    let content = std::fs::read_to_string(path)?;
    let config: TopicsConfig = toml::from_str(&content)?;
    Ok(config)
}
