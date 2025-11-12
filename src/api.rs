use leptos::{prelude::{ServerFnError, use_context}, server};
use crate::types::{Mod, Pack};

#[server]
pub async fn fetch_pack(pack: String) -> Result<Pack, ServerFnError> {
    use sqlx::{Row, query};

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
    "#).bind(pack).fetch_all(&state.pool).await.unwrap();
    let pack = Pack {
        name: rows[0].get("name"),
        mods: rows.iter().map(|r| Mod { slug: r.get("slug"), votes: r.get("votes") }).collect(),
    };

    Ok(pack)
}
