use crate::appstate::AppState;
use crate::handler::{doc, menumgmt, ordermgmt};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tracing::info;

pub async fn init_router(state: Arc<AppState>) -> Router {
    let app = Router::new()
        .nest(
            "/api/v1/:table",
            Router::new().nest(
                "/order",
                Router::new()
                    .route("/add", post(ordermgmt::add_order))
                    .route("/view", get(ordermgmt::view_order))
                    .route("/delete", delete(ordermgmt::delete_order))
                    .route("/bill", get(ordermgmt::get_table_bill))
                    .route("/settle", post(ordermgmt::settle_order))
                    .nest(
                        "/item",
                        Router::new()
                            .route("/get", get(ordermgmt::get_item))
                            .route("/delete", delete(ordermgmt::delete_item_from_order))
                            .route("/add", post(ordermgmt::add_item_to_order)),
                    ),
            ),
        )
        .nest(
            "/api/v1/menu",
            Router::new()
                .route("/view", get(menumgmt::fetch_menu))
                .nest(
                    "/item",
                    Router::new()
                        .route("/add", post(menumgmt::add_items_to_menu))
                        .route("/delete", post(menumgmt::delete_items_from_menu))
                        .route("/update", put(menumgmt::update_items_of_menu)),
                ),
        )
        .route("/api/v1/api-doc", get(doc::get_api_doc))
        .with_state(state);
    info!("Router: Routes are initialized");
    return app;
}
