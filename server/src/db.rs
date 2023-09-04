use crate::args::Args;
use crate::request::Menu;
use anyhow::Result;
use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Pool,
};
use std::{fs::File, io::BufReader, path::Path};
use tracing::info;

async fn init_sqlx_pool(args: &Args) -> Result<Pool<Postgres>> {
    let db_url = match args.db_password.is_empty() {
        true => {
            format!(
                "postgress://{}@{}:{}/{}",
                args.db_user, args.db_host, args.db_port, args.db_name
            )
        }
        false => {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                args.db_user, args.db_password, args.db_host, args.db_port, args.db_name
            )
        }
    };
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|e| e.into())
}

async fn create_table_for_menu(file: String, db_conn_pool: &Pool<Postgres>) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS Menu (
            item_name VARCHAR NOT NULL PRIMARY KEY,
            item_description TEXT,
            price FLOAT NOT NULL
        );
    "#,
    )
    .execute(db_conn_pool)
    .await
    .unwrap();

    let yaml_reader =
        BufReader::new(File::open(Path::new(&file)).expect("Failed to open menu yaml file"));
    let menu: Menu = serde_yaml::from_reader(yaml_reader).unwrap();
    for item in menu.items {
        let exists = sqlx::query(r#"SELECT 1 FROM Menu WHERE item_name = $1"#)
            .bind(item.name.clone())
            .fetch_optional(db_conn_pool)
            .await
            .unwrap()
            .is_some();
        if !exists {
            sqlx::query(
                r#"
            INSERT INTO Menu (item_name, item_description, price)
            VALUES ($1, $2, $3)
            "#,
            )
            .bind(item.name)
            .bind(item.description)
            .bind(item.price)
            .execute(db_conn_pool)
            .await
            .expect("Failed to insert data");
        }
    }
    info!("DB: Created Table: Menu");
}

async fn create_table_for_orders(db_conn_pool: &Pool<Postgres>) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS Orders (
            id SERIAL PRIMARY KEY,
            table_no INT NOT NULL,
            item_name VARCHAR NOT NULL,
            quantity OID NOT NULL,
            preparation_time OID NOT NULL,
            status VARCHAR NOT NULL
        );
    "#,
    )
    .execute(db_conn_pool)
    .await
    .unwrap();
    info!("DB: Created Table: Orders");
}

pub async fn init_db(args: &Args) -> Pool<Postgres> {
    let db_conn_pool = init_sqlx_pool(args)
        .await
        .expect("failed to connect to database");

    // Create Table : "Menu"
    // Parse the menu.yaml file and store the items to Menu Table
    create_table_for_menu(args.menu_yaml_file.clone(), &db_conn_pool).await;

    // Create Table: "Orders"
    create_table_for_orders(&db_conn_pool).await;
    info!("DB: Database initialized");
    return db_conn_pool;
}
