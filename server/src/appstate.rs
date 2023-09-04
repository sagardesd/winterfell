use crate::args::Args;
use crate::db;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    pub db_conn_pool: Pool<Postgres>,
    //pub app_id: String,
}

pub async fn init_app_state(args: &Args) -> Arc<AppState> {
    let db_conn_pool = db::init_db(args).await;
    Arc::new(AppState { db_conn_pool })
    /*
    Arc::new(AppState {
        app_id: String::from("test"),
    })
    */
}
