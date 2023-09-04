use crate::appstate::AppState;
use crate::request::*;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::Row;
use std::sync::Arc;
use tracing::{error, info};

pub async fn fetch_menu(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match sqlx::query(
        r#"
        SELECT * from Menu 
        "#,
    )
    .fetch_all(&state.db_conn_pool)
    .await
    {
        Ok(rows) => {
            let mut items: Vec<MenuItem> = Vec::new();
            for row in rows {
                items.push(MenuItem {
                    name: row.get("item_name"),
                    description: row.get("item_description"),
                    price: row.get("price"),
                });
            }
            (StatusCode::OK, Json(Menu { items })).into_response()
        }
        Err(e) => {
            error!("fetch_menu: Failed to fetch menu content with error {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch the menu content from DB"),
            )
                .into_response()
        }
    }
}

pub async fn add_items_to_menu(
    State(state): State<Arc<AppState>>,
    Json(items): Json<Vec<MenuItem>>,
) -> impl IntoResponse {
    let transaction = state.db_conn_pool.begin().await.unwrap();
    for item in items {
        sqlx::query(
            r#"
                INSERT into Menu (item_name, item_description, price)
                VALUES($1, $2, $3)
            "#,
        )
        .bind(item.name)
        .bind(item.description)
        .bind(item.price)
        .execute(&state.db_conn_pool)
        .await
        .unwrap();
    }
    match transaction.commit().await {
        Ok(_) => {
            info!("add_items_to_menu: Successfully added the items to menu");
            (
                StatusCode::OK,
                format!("Succefully update menu with the items"),
            )
                .into_response()
        }
        Err(e) => {
            error!(
                "add_items_to_menu: Failed to add items to Menu table of DB with error {}",
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to add items to Menu"),
            )
                .into_response()
        }
    }
}

pub async fn delete_items_from_menu(
    State(state): State<Arc<AppState>>,
    Json(items): Json<Vec<MenuItem>>,
) -> impl IntoResponse {
    let transaction = state.db_conn_pool.begin().await.unwrap();
    for item in items {
        match sqlx::query(
            r#"
                DELETE from Menu Where item_name = $1
            "#,
        )
        .bind(item.name)
        .execute(&state.db_conn_pool)
        .await
        {
            Ok(_) => {
                continue;
            }
            Err(e) => {
                error!(
                    "delete_items_from_order: db execute failed with error {}",
                    e
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to delete items from menu"),
                )
                    .into_response();
            }
        }
    }
    match transaction.commit().await {
        Ok(_) => {
            info!("add_items_to_menu: Successfully added the items to menu");
            (
                StatusCode::OK,
                format!("Succefully update menu with the items"),
            )
                .into_response()
        }
        Err(e) => {
            error!(
                "add_items_to_menu: Failed to add items to Menu table of DB with error {}",
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to add items to Menu"),
            )
                .into_response()
        }
    }
}

pub async fn update_items_of_menu(
    State(state): State<Arc<AppState>>,
    Json(items): Json<Vec<MenuItemUpdate>>,
) -> impl IntoResponse {
    let transaction = state.db_conn_pool.begin().await.unwrap();
    for item in items {
        let _ = sqlx::query(
            r#"
            UPDATE Menu SET price = $1 WHERE item_name = $2
            "#,
        )
        .bind(item.price)
        .bind(item.name)
        .execute(&state.db_conn_pool)
        .await
        .map_err(|e| {
            error!("udpate_items_of_menu: Failed with error {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update the menu items"),
            )
                .into_response();
        });
    }
    match transaction.commit().await {
        Ok(_) => {
            info!("update_items_to_menu: Successfully added the items to menu");
            (
                StatusCode::OK,
                format!("Succefully update menu with the items"),
            )
                .into_response()
        }
        Err(e) => {
            error!(
                "update_items_to_menu: Failed to update items to Menu table of DB with error {}",
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update items to Menu"),
            )
                .into_response()
        }
    }
}
