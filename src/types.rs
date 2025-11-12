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
pub struct FullMod {
    title: String,
    description: String,
}
