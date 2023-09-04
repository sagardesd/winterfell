use axum::{http::StatusCode, response::IntoResponse};
pub async fn get_api_doc() -> impl IntoResponse {
    let docs = "POST   /api/v1/:table/order/add         : Place an order for a table\n\
                GET    /api/v1/:table/order/view        : Fetch the order details of that table\n\
                GET    /api/v1/:table/order/bill              : Get the final bill of that table\n\
                POST   /api/v1/:table/order/settle      : Settle the order change the status to done\n\
                DELETE /api/v1/:table/order/delete      : Remove an order for the table\n\
                GET    /api/v1/:table/order/item/get    : Get the details of an item for an order of a table\n\
                POST    /api/v1/:table/order/item/add    : Add an item to the order of a table\n\
                DELETE /api/v1/:table/order/item/delete : Remove an item from the order of a table\n\
                GET    /api/v1/menu/view                : Get the Menu\n\
                POST   /api/v1/menu/add                 : Add new items to menu\n\
                PUT    /api/v1/menu/update              : Update the existing item details of the menu\n\
                DELETE /api/v1/menu/delete              : Delete items from the Menu\n\
                GET    /health                          : Get server health\n\
                GET    /api/v1/api-doc                  : Get this page\n";
    (StatusCode::OK, docs.to_string())
}
