use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

type DB = Pool<Postgres>;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub leptos_options: leptos::config::LeptosOptions,
    pub db_pool: DB,
}
