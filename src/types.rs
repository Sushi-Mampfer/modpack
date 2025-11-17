use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::sqlite::SqlitePool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pack {
    pub name: String,
    pub version: String,
    pub loader: String,
    pub mods: Vec<Mod>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mod {
    pub slug: String,
    pub votes: i32,
}

#[derive(Deserialize, Clone)]
pub struct WebMod {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub icon_url: String,
}

#[derive(Clone)]
pub struct FullMod {
    pub pack: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub votes: i32,
}

#[derive(Deserialize, Clone)]
pub struct ModSearch {
    pub hits: Vec<WebMod>,
}

#[derive(Deserialize, Clone)]
pub struct Dependencies {
    pub projects: Vec<WebMod>,
}
