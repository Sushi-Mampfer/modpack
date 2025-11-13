use crate::types::{Mod, Pack};
use chrono::Utc;
use leptos::{
    prelude::{use_context, ServerFnError},
    server,
};
#[cfg(feature = "ssr")]
use sqlx::{query, Row};

#[server]
pub async fn fetch_pack(pack: String) -> Result<Pack, ServerFnError> {
    let state = use_context::<crate::types::AppState>().unwrap();

    let rows = query(
        r#"
        SELECT 
            p.name,
            m.slug,
            SUM(CASE v.upvote WHEN 1 THEN 1 WHEN 0 THEN -1 ELSE 0 END) as votes
        FROM packs p
        LEFT JOIN mods m ON p.id = m.pack
        LEFT JOIN votes v ON p.id = v.pack AND m.slug = v.slug
        WHERE p.id = ?
        GROUP BY m.slug
    "#,
    )
    .bind(pack)
    .fetch_all(&state.pool)
    .await
    .unwrap();
    let pack = Pack {
        name: rows[0].get("name"),
        mods: rows
            .iter()
            .map(|r| Mod {
                slug: r.get("slug"),
                votes: r.get("votes"),
            })
            .collect(),
    };

    Ok(pack)
}

#[server]
pub async fn upvote(pack: String, slug: String) -> Result<(), ServerFnError> {
    let state = use_context::<crate::types::AppState>().unwrap();

    query(
        r#"
        INSERT OR REPLACE INTO votes (pack, slug, time, upvote, ip)
        VALUES (?, ?, ?, 1, "127.0.0.1")
    "#,
    )
    .bind(&pack)
    .bind(&slug)
    .bind(Utc::now().timestamp())
    .execute(&state.pool)
    .await
    .unwrap();
    Ok(())
}

#[server]
pub async fn downvote(pack: String, slug: String) -> Result<(), ServerFnError> {
    let state = use_context::<crate::types::AppState>().unwrap();

    query(
        r#"
        INSERT OR REPLACE INTO votes (pack, slug, time, upvote, ip)
        VALUES (?, ?, ?, -1, ?)
    "#,
    )
    .bind(&pack)
    .bind(&slug)
    .bind(Utc::now().timestamp())
    .bind(get_ip())
    .execute(&state.pool)
    .await
    .unwrap();
    Ok(())
}

#[server]
pub async fn remove_vote(pack: String, slug: String) -> Result<(), ServerFnError> {
    let state = use_context::<crate::types::AppState>().unwrap();

    query(
        r#"
        DELETE FROM votes
        WHERE pack = ? AND slug = ? AND ip = ?
    "#,
    )
    .bind(&pack)
    .bind(&slug)
    .bind(get_ip())
    .execute(&state.pool)
    .await
    .unwrap();
    Ok(())
}

fn get_ip() -> String {
    "127.0.0.1".to_string()
}
