pub mod app;
pub mod pages;
pub mod types;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use crate::{app::*, types::AppState};
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::{query, sqlite::SqliteConnectOptions, SqlitePool};
    use std::{env, str::FromStr};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::from_str("db.sqlite")
            .unwrap()
            .create_if_missing(true),
    )
    .await
    .unwrap();

    query(
        r#"
        CREATE TABLE IF NOT EXISTS packs (
	        id	TEXT NOT NULL,
	        name    TEXT NOT NULL,
	        admin	TEXT NOT NULL,
	        PRIMARY KEY(id)
        )
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    query(
        r#"
        CREATE TABLE mods (
	        pack	TEXT NOT NULL,
	        slug	TEXT NOT NULL,
	        time	INTEGER NOT NULL,
	        ip	TEXT
        )
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(AppState { pool: pool.clone() }),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or_else(|_| "8080".to_string())
    ))
    .await
    .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
