use crate::types::{Mod, Pack};
use chrono::Utc;
use leptos::{
    prelude::{use_context, ServerFnError},
    server,
};
use leptos_axum::extract;
#[cfg(feature = "ssr")]
use {
    axum::http::HeaderMap,
    rand::{distr::{Alphabetic, SampleString}, rng},
    sqlx::{query, Row}
};

#[server]
pub async fn create_pack(name: String, loader: String, version: String) -> Result<(), ServerFnError> {
    let state = use_context::<crate::types::AppState>().unwrap();

    for _ in 0..100 {
        let pack = Alphabetic.sample_string(&mut rng(), 10);
        let admin = Alphabetic.sample_string(&mut rng(), 10);
        let res = query(
        r#"
            INSERT INTO packs (id, name, admin, version, loader)
            VALUES (?, ?, ?, ?, ?)
        "#).bind(&pack).bind(&name).bind(&admin).bind(&version).bind(&loader).execute(&state.pool).await;
        match res {
            Ok(_) => {
                query(
                r#"
                    INSERT OR IGNORE INTO mods (pack, slug, time, ip)
                    VALUES (?, "", ?, ?)
                "#).bind(&pack).bind(Utc::now().timestamp()).bind(get_ip().await).execute(&state.pool).await.unwrap();
                leptos_axum::redirect(&format!("/pack/{}/{}", pack, admin));
                return Ok(())
            },
            Err(_) => continue,
        }
    }
    Err(ServerFnError::ServerError("No space left on device.".to_string()))
}

#[server]
pub async fn add_mods(pack: String, slugs: Vec<String>) -> Result<(), ServerFnError> {
    if slugs.len() == 0 {return Ok(());}
    let state = use_context::<crate::types::AppState>().unwrap();

    let ip = get_ip().await;
    let values = vec!["(?, ?, ?, ?)"; slugs.len()].join(", ");
    let query_text = &format!(r#"
        INSERT OR IGNORE INTO mods (pack, slug, time, ip)
        VALUES {}
    "#, values);
    let mut query = query(&query_text);
    for s in slugs {
        query = query.bind(&pack).bind(s).bind(Utc::now().timestamp()).bind(&ip);
    }
    query.execute(&state.pool).await.unwrap();
    Ok(())
}

#[server]
pub async fn fetch_pack(pack: String) -> Result<Pack, ServerFnError> {
    let state = use_context::<crate::types::AppState>().unwrap();

    let rows = query(
        r#"
        SELECT 
            p.name,
            p.version,
            p.loader,
            m.slug,
            SUM(CASE WHEN v.upvote > 0 THEN 1 WHEN v.upvote < 0 THEN -1 ELSE 0 END) as votes
        FROM packs p
        LEFT JOIN mods m ON p.id = m.pack
        LEFT JOIN votes v ON p.id = v.pack AND m.slug = v.slug
        WHERE p.id = ?
        GROUP BY p.name, p.version, p.loader, m.slug
    "#,
    )
    .bind(pack)
    .fetch_all(&state.pool)
    .await
    .unwrap();
    
    let pack = Pack {
        name: rows[0].get("name"),
        version: rows[0].get("version"),
        loader: rows[0].get("loader"),
        mods: rows
            .iter()
            .filter_map(|r| {
                let slug: String = r.get("slug");
                if slug.is_empty() {
                    None
                } else {
                    Some(Mod {
                        slug,
                        votes: r.get("votes"),
            })}})
            .collect(),
    };
    Ok(pack)
}

#[server]
pub async fn add_mod(pack: String, slug: String) -> Result<(), ServerFnError> {
    let state = use_context::<crate::types::AppState>().unwrap();

    query(
        r#"
        INSERT OR IGNORE INTO mods (pack, slug, time, ip)
        VALUES (?, ?, ?, ?)
    "#,
    )
    .bind(&pack)
    .bind(&slug)
    .bind(Utc::now().timestamp())
    .bind(get_ip().await)
    .execute(&state.pool)
    .await
    .unwrap();
    Ok(())
}

#[server]
pub async fn remove_mod(pack: String, admin: String, slug: String) -> Result<(), ServerFnError> {
    let state = use_context::<crate::types::AppState>().unwrap();

    query(r#"
        DELETE FROM mods
        WHERE (EXISTS(SELECT 1 FROM packs WHERE id = ? and admin = ?) = 1 AND pack = ? and slug = ?)
    "#).bind(&pack).bind(&admin).bind(&pack).bind(&slug).execute(&state.pool).await.unwrap();
    query(r#"
        DELETE FROM votes
        WHERE (EXISTS(SELECT 1 FROM packs WHERE id = ? and admin = ?) = 1 AND pack = ? and slug = ?)
    "#).bind(&pack).bind(&admin).bind(&pack).bind(&slug).execute(&state.pool).await.unwrap();
    Ok(())
}

#[server]
pub async fn upvote(pack: String, slug: String) -> Result<(), ServerFnError> {
    let state = use_context::<crate::types::AppState>().unwrap();

    query(
        r#"
        INSERT OR REPLACE INTO votes (pack, slug, time, upvote, ip)
        VALUES (?, ?, ?, 1, ?)
    "#,
    )
    .bind(&pack)
    .bind(&slug)
    .bind(Utc::now().timestamp())
    .bind(get_ip().await)
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
    .bind(get_ip().await)
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
    .bind(get_ip().await)
    .execute(&state.pool)
    .await
    .unwrap();
    Ok(())
}

async fn get_ip() -> String {
    let headers: HeaderMap = extract().await.unwrap();
    headers.get("X-Forwarded-For").unwrap().to_str().unwrap().to_owned()
}