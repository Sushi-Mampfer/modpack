use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::SqlitePool;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

#[derive(Serialize, Deserialize)]
pub struct Pack {}
