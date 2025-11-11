use leptos::{prelude::ServerFnError, server};

use crate::types::Pack;

#[server]
pub async fn fetch_pack(pack: String) -> Result<Pack, ServerFnError> {
    Err(ServerFnError::ServerError("testing".to_string()))
}
