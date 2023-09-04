use clap::Parser;
use restaurant::appstate;
use restaurant::args::Args;
use restaurant::router;
use std::sync::Arc;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};
use tracing::{debug, info, warn};
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() {
    // Setup args
    let args = Args::parse();
    debug!("Application startup args: {:?}", args);
    let builder = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_level(true)
        .with_ansi(args.enable_dev_log_format);
    if args.enable_dev_log_format {
        builder.pretty().init();
    } else {
        builder.without_time().compact().init();
    }

    // Initialize Application state
    let state = appstate::init_app_state(&args).await;
    // Initialize Routes
    let app = router::init_router(Arc::clone(&state)).await;
    // Start Server
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::from_str(&args.bind_address).unwrap()),
        args.listen_port,
    );
    info!("Server listening on {}", addr);
    let server = axum::Server::bind(&addr);
    let docs = "POST   /api/v1/:table/order/add         : Place an order for a table\n\
                GET    /api/v1/:table/order/view        : Fetch the order details of that table\n\
                GET    /api/v1/:table/bill              : Get the final bill of that table\n\
                POST   /api/v1/:table/order/settle      : Settle the order change the status to done\n\
                DELETE /api/v1/:table/order/delete      : Remove an order for the table\n\
                GET    /api/v1/:table/order/item/get    : Get the details of an item for an order of a table\n\
                POST   /api/v1/:table/order/item/add    : Add a item to the order of the table\n\
                DELETE /api/v1/:table/order/item/delete : Remove an item from the order of a table\n\
                GET    /api/v1/menu/view                : Get the Menu\n\
                POST   /api/v1/menu/add                 : Add new items to menu\n\
                PUT    /api/v1/menu/update              : Update the existing item details of the menu\n\
                DELETE /api/v1/menu/delete              : Delete items from the Menu\n\
                GET    /health                          : Get server health\n\
                GET    /api/v1/api-doc                  : Get this page\n";
    info!("ENDPOINTS:\n{}", docs);
    tokio::select! {
        _ = server.serve(app.into_make_service()) => {
            warn!("Server is stopped")
        }
    }
}
