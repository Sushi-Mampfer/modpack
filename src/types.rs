use reqwest::get;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::sqlite::SqlitePool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pack {
    pub name: String,
    pub mods: Vec<Mod>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Mod {
    pub slug: String,
    pub votes: i32,
}

#[derive(Deserialize)]
pub struct WebMod {
    pub title: String,
    pub description: String,
}

#[derive(Clone)]
pub struct FullMod {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub votes: i32,
}

impl FullMod {
    pub async fn from_mod(old_mod: Mod) -> Option<Self> {
        let res = get(format!("https://api.modrinth.com/v2/project/{}", old_mod.slug)).await.ok()?;
        let web_mod: WebMod = res.json().await.ok()?;
        Some(Self { slug: old_mod.slug, title: web_mod.title, description: web_mod.description, votes: old_mod.votes })
    }
}