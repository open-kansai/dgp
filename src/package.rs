use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PackType<T> {
    Include(T),
    Exclude,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordGuildPackage {
    /// Package version
    pub version: u8,
    /// Source guild id
    pub guild_id: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub pack: GuildPack,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildPack {
    pub name: PackType<String>,
}
