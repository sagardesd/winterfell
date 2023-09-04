use crate::appstate::AppState;
use crate::request::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rand::Rng;
use sqlx::Row;
use std::sync::Arc;
use tracing::{info, error};
use std::collections::HashMap;

pub async fn add_order(
        State(state): State<Arc<AppState>>,
        Path(table): Path<u32>,
        Json(order): Json<Order>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/{}/order/add", table); 
        info!("Add: Table: {:?} Order: {:?}", table, order);
        let transaction = state.db_conn_pool.begin().await.unwrap();
        for item in order.items {
            let random_minute = rand::thread_rng().gen_range(5..16);
            let prep_time = item.item_quantity * random_minute;
            sqlx::query(
                r#"
                    INSERT into Orders (table_no, item_name, quantity, preparation_time, status)
                    VALUES($1, $2, $3, $4, $5)
                    "#,
            )
            .bind(table)
            .bind(item.item_name)
            .bind(item.item_quantity)
            .bind(prep_time)
            .bind(String::from("InProgress"))
            .execute(&state.db_conn_pool)
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();
        info!("Add: Order added successfully to table no: {}", table,);
        (
            StatusCode::OK,
            Json(OrderStatus {
                status: String::from("InProgress"),
            }),
        )
            .into_response()
    }

    pub async fn view_order(
        State(state): State<Arc<AppState>>,
        Path(table): Path<u32>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/{}/order/view", table);
        match sqlx::query(
                r#"
                SELECT * from Orders WHERE table_no = $1 AND status = $2
                "#,
            )
            .bind(table)
            .bind(String::from("InProgress"))
            .fetch_all(&state.db_conn_pool)
            .await
        {
            Ok(rows) => {
                let mut items: Vec<Item> = Vec::new();
                for row in rows {
                    items.push(Item {
                        item_name: row.get("item_name"),
                        item_quantity: row.get("quantity"),
                    })
                }
                (
                    StatusCode::OK,
                    Json(Table {
                        table_no: table,
                        order: Order { items },
                    }),
                )
                    .into_response()
            }
            Err(e) => match e {
                sqlx::error::Error::RowNotFound => (
                    StatusCode::NOT_FOUND,
                    String::from("Could not found order details"),
                )
                    .into_response(),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Faile to fetch order details"),
                )
                    .into_response(),
            },
        }
    }

    pub async fn delete_order(
        State(state): State<Arc<AppState>>,
        Path(table): Path<u32>,
    ) -> impl IntoResponse {
        info!("DELETE /api/v1/{}/order/delete", table);
        match sqlx::query(
            r#"
                DELETE from Orders WHERE table_no = $1 AND status = $2
        "#,
        )
        .bind(table)
        .bind(String::from("InProgress"))
        .execute(&state.db_conn_pool)
        .await
        {
            Ok(_) => (StatusCode::OK, String::from("Successfully removed order")).into_response(),
            Err(e) => match e {
                sqlx::error::Error::RowNotFound => (
                    StatusCode::NOT_FOUND,
                    String::from("Could not found order details"),
                )
                    .into_response(),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Faile to remove order details"),
                )
                    .into_response(),
            },
        }
    }

    pub async fn get_table_bill(
        State(state): State<Arc<AppState>>,
        Path(table): Path<u32>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/{}/order/bill", table);
        let result = sqlx::query(r#"
                SELECT COALESCE(SUM(order_item.quantity::bigint::float8 * menu_item.price), 0) AS total_bill
                FROM Orders order_item
                INNER JOIN Menu menu_item ON order_item.item_name = menu_item.item_name
                WHERE order_item.status = 'InProgress' AND order_item.table_no = $1
            "#,)
            .bind(table)
            .fetch_one(&state.db_conn_pool)
            .await;
        match result {
            Ok(pgrow) => {
                let total_bill: f64 = pgrow.get("total_bill");
                (StatusCode::OK, Json(TableBill {
                    table_no: table,
                    total_bill: total_bill,
                })).into_response()
            }
            Err(e) => {
                error!("Failed to fetch bill for table_no: {} with error {}", table, e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch bill for table_no {}", table)).into_response()
            }

        }
    }

    pub async fn settle_order(
        state: State<Arc<AppState>>,
        Path(table): Path<u32>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/{}/order/settle", table);
        info!("settle_order: Settle order request for table_no: {}", table);
        match sqlx::query(
            r#"
                UPDATE Orders SET status = 'Done' WHERE table_no = $1 AND status = 'InProgress'
            "#,
        )
        .bind(table)
        .execute(&state.db_conn_pool)
        .await
        {
            Ok(_) => (
                StatusCode::OK,
                format!("Orders are settled for the table_no: {}", table),
            )
                .into_response(),
            Err(e) => match e {
                sqlx::error::Error::RowNotFound => (
                    StatusCode::NOT_FOUND,
                    format!("Could not found any inprogress order for the table_no: {}", table),
                )
                    .into_response(),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Could not able to settle the order of the table_no: {}", table),
                )
                    .into_response(),
            },
        }
    }
        pub async fn get_item(
            State(state): State<Arc<AppState>>,
            Path(table): Path<u32>,
            Query(item): Query<HashMap<String, String>>,
        ) -> impl IntoResponse {
            info!("GET /api/v1/{}/order/item", table);
            if item.get("item_name").is_none() {
                return (StatusCode::BAD_REQUEST, format!("Please pass item_name in query paramater")).into_response();
            }
            let item_name = item.get("item_name").unwrap().clone();
            info!("get_item: table: {} item_name: {:?}", table, item_name);
            match sqlx::query(r#"
                SELECT * from Orders Where table_no = $1 AND status = 'InProgress' AND item_name = $2
                "#)
                .bind(table)
                .bind(item_name.clone())
                .fetch_one(&state.db_conn_pool)
                .await 
            {
                    Ok(row) => {

                        let payload = GetItemResponse {
                            table_no: table,
                            name: item_name,
                            quantity: row.get("quantity"),
                            preparation_time: row.get("preparation_time")
                        };
                        info!("get_item: item fetched from db: {:?}", payload);
                        (StatusCode::OK, Json(payload)).into_response()
                    }
                    Err(e) => {
                        error!("get_item: Failed to fetch item details for table_no: {}, item_name: {} error: {}", table, item_name, e.to_string());
                        (StatusCode::NOT_FOUND, String::from("Item not found")).into_response()
                    }
                }
        }

        pub async fn delete_item_from_order(
            State(state): State<Arc<AppState>>,
            Path(table): Path<u32>,
            Query(item): Query<HashMap<String, String>>
        ) -> impl IntoResponse {
            info!("DELETE /api/v1/{}/order/item/delete", table);
            if item.get("item_name").is_none() {
                return (StatusCode::BAD_REQUEST, format!("Failed since item_name is not passed")).into_response();
            }
            let item_name = item.get("item_name").unwrap().clone();
            match sqlx::query(r#"
                DELETE from Orders Where table_no = $1 AND status = 'InProgress' And item_name = $2
                "#)
                .bind(table)
                .bind(item_name.clone())
                .execute(&state.db_conn_pool)
                .await 
            {
                Ok(_) => {
                    let resp = format!("For table {} removed item {}", table, item_name);
                    return (StatusCode::OK, resp).into_response();
                }
                Err(e) => {
                    error!("delete_item_from_order: Failed to delete item {} for table {}", table, item_name);
                    match e {
                        sqlx::error::Error::RowNotFound => 
                            (
                                StatusCode::NOT_FOUND,
                                format!("Failed to delete the item {} for table {}", item_name, table),
                            ).into_response(),
                        _ => (
                            StatusCode::CONFLICT,
                            String::from("Could not able to settle the order of the table"),
                            ).into_response(),
                    }
                }
            }
        }

        pub async fn add_item_to_order(
            State(state): State<Arc<AppState>>,
            Path(table): Path<u32>,
            Json(item): Json<Item>
        ) -> impl IntoResponse {
            info!("POST /api/v1/{}/order/item/add", table);
            let random_minute = rand::thread_rng().gen_range(5..16);
            let prep_time = item.item_quantity * random_minute;
            match sqlx::query(
                        r#"
                            INSERT into Orders (table_no, item_name, quantity, preparation_time, status)
                            VALUES($1, $2, $3, $4, $5)
                        "#)
                    .bind(table)
                    .bind(item.item_name.clone())
                    .bind(item.item_quantity)
                    .bind(prep_time)
                    .bind(String::from("InProgress"))
                    .execute(&state.db_conn_pool).await {
                    Ok(_) => {
                        (StatusCode::OK, format!("successfully added item {:?} for table {:?}", item, table)).into_response()
                    }
                    Err(e) => {
                        error!("add_item_to_order: Failed to add item: {:?} to order of table {:?} error: {:?}", item, table, e);
                        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to add item to order of table {}", table)).into_response()
                    }
            }
        }
