use clap::Parser;
use futures;
use serde::{Deserialize, Serialize};
use tokio;
use tokio::task::JoinHandle;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Table {
    pub table_no: u32,
    pub order: Order,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Order {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub item_name: String,
    pub item_quantity: u32,
}

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    #[clap(
        long,
        env = "RUST_LOG",
        default_value = "info",
        help = "Log level. dubug, info, warn, error"
    )]
    pub log_level: String,

    #[clap(long, env = "ENABLE_DEV_LOG_FORMAT", default_value = "true")]
    pub enable_dev_log_format: bool,

    #[clap(long, env = "SERVER_IP", default_value = "127.0.0.1:8081")]
    pub server_ip: String,
}

pub async fn place_order_and_verify(server_ip: String, table: Table, item: Item) {
    let http_client = reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            );
            headers
        })
        .build()
        .unwrap();
    // Add item
    let add_order_endpoint = format!("/api/v1/{}/order/add", table.table_no);
    let order_add_resp = match serde_json::to_string(&table.order) {
        Ok(payload) => {
            let response = http_client
                .post(format!("http://{}{}", server_ip, add_order_endpoint))
                .body(payload)
                .send()
                .await
                .unwrap();
            response
        }
        Err(e) => {
            panic!("Failed to place order {:?}  with error {:?}", table, e);
        }
    };
    info!(
        "Table_no: {} add_order_resp: {:?}",
        table.table_no, order_add_resp
    );
    // View order
    let view_order_endpoint = format!("/api/v1/{}/order/view", table.table_no);
    let order_view_resp = http_client
        .get(format!("http://{}{}", server_ip, view_order_endpoint))
        .send()
        .await
        .unwrap();
    info!(
        "Table_no: {} order_view_resp: {:?}",
        table.table_no, order_view_resp
    );
    // Get item
    for item in table.order.items {
        let get_item_endpoint = format!(
            "/api/v1/{}/order/item/get?item_name={}",
            table.table_no, item.item_name
        );
        let get_item_resp = http_client
            .get(format!("http://{}{}", server_ip, get_item_endpoint))
            .send()
            .await
            .unwrap();
        info!(
            "Table_no: {} get_item_resp: {:?}",
            table.table_no, get_item_resp
        );
    }
    // Add item
    let add_item_endpoint = format!("/api/v1/{}/order/item/add", table.table_no);
    let add_item_resp = http_client
        .post(format!("http://{}{}", server_ip, add_item_endpoint))
        .body(serde_json::to_string(&item).unwrap())
        .send()
        .await
        .unwrap();
    info!(
        "Table_no: {} add_item_resp: {:?}",
        table.table_no, add_item_resp
    );
    // Delete item
    let delete_item_endpoint = format!(
        "/api/v1/{}/order/item/delete?item_name={}",
        table.table_no, item.item_name
    );
    let delete_item_resp = http_client
        .delete(format!("http://{}{}", server_ip, delete_item_endpoint))
        .send()
        .await
        .unwrap();
    info!(
        "Table_no: {} delete_item_resp: {:?}",
        table.table_no, delete_item_resp
    );

    // Get bill for a table
    let get_bill_endpoint = format!("/api/v1/{}/order/bill", table.table_no);
    let order_bill_resp = http_client
        .get(format!("http://{}{}", server_ip, get_bill_endpoint))
        .send()
        .await
        .unwrap();
    info!(
        "Table_no: {} order_bill_resp: {:?}",
        table.table_no, order_bill_resp
    );
    // Settle order for a table
    let set_table_endpoint = format!("/api/v1/{}/order/settle", table.table_no);
    let settle_table_resp = http_client
        .post(format!("http://{}{}", server_ip, set_table_endpoint))
        .send()
        .await
        .unwrap();
    info!(
        "Table_no: {} settle_table_resp: {:?}",
        table.table_no, settle_table_resp
    );
}

#[tokio::main]
async fn main() {
    // Setup args
    let args = Args::parse();
    let builder = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_level(true)
        .with_ansi(args.enable_dev_log_format);
    if args.enable_dev_log_format {
        builder.pretty().init();
    } else {
        builder.without_time().compact().init();
    }
    let mut items: Vec<Item> = Vec::new();
    items.push(Item {
        item_name: String::from("Item1"),
        item_quantity: 2,
    });
    items.push(Item {
        item_name: String::from("Item2"),
        item_quantity: 1,
    });
    let item_op = Item {
        item_name: String::from("Item3"),
        item_quantity: 3,
    };
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for table_no in 1..11 {
        let table = Table {
            table_no,
            order: Order {
                items: items.clone(),
            },
        };
        handles.push(tokio::spawn(place_order_and_verify(
            args.server_ip.clone(),
            table,
            item_op.clone(),
        )))
    }
    let results = futures::future::join_all(handles).await;
    for result in results {
        info!("{:?}", result);
    }
}
