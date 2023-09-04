# Winterfell

This repo is a basic Restaurant order management system written in RUST.

## Server
The api server is written in Rust using Axum crate and the server is using PostgreSql as Database.

Upon start the api server loads the Menu from the data/sample_menu.yaml file to the Database and creates the table Menu.

You can clone the repo and add more item/udpate item to the data/sample_menu.yaml.

User application can view the menu by using "GET /api/v1/menu/view" api.

User application should then select only the item in the menu to place order for a table.

Please refer the server/src/args.rs to know the details of the arguments and enviornment variables the server is using during startup.

**How to start the Server locally**
```
Pre-requisite:
- Docker must be installed on your machine.

Steps to Start Server:
1. Clone the Repo
2. cd Server
3. docker compose up
```

docker compose up will start the api server and PostgreSql db in two container.
The services ips:
```
API Server will be listening on     : 127.0.0.1:8081
The DB service will be listening on : 127.0.0.1:5432
```

**The api server hosts the below Endpoints:**
```
                                        ORDER MANAGEMENT APIS

POST   /api/v1/:table/order/add         : Place an order for a table
                                          Sample Payload : '{"items":[{"item_name":"Item1", "item_quantity":2},{"item_name":"Item2", "item_quantity": 1}]}'
                                          Example: curl -v --request POST "http://127.0.0.1:8081/api/v1/124/order/add" --header "Content-Type: application/json" -d 
                                          '{"items":[{"item_name":"Item1", "item_quantity":2},{"item_name":"Item2", "item_quantity": 1}]}'                                          

GET    /api/v1/:table/order/view        : Fetch the order details of that table

GET    /api/v1/:table/bill              : Get the final bill of that table

POST    /api/v1/:table/order/settle     : Settle the order change the status to done

DELETE /api/v1/:table/order/delete      : Remove an order for the table

GET    /api/v1/:table/order/item/get    : Get the details of an item for an order of a table
                                          Pass the query as "item_name"="name_of_item_you_want_details"
                                          Example: curl -v --request GET "http://127.0.0.1:8081/api/v1/124/order/item/get?item_name=Item1"

POST   /api/v1/:table/order/item/add    : Add a item to the order of the table
                                          Sample Payload: '{"item_name":"Item1", "item_quantity":2}'
                                          Example: curl -v --request POST "http://127.0.0.1:8081/api/v1/124/order/item/add" --header "Content-Type: application/json" -d 
                                          '{"item_name":"Item1", "item_quantity":2}'

DELETE /api/v1/:table/order/item/delete : Remove an item from the order of a table
                                          Pass the query as "item_name"="item_name_you_want_to_delete"
                                          Example: curl -v --request DELETE "http://127.0.0.1:8081/api/v1/124/order/item/delete?item_name=Item1"


                                        MENU MANAGEMENT APIS

GET    /api/v1/menu/view                : Get the Menu

POST   /api/v1/menu/add                 : Add new items to menu

PUT    /api/v1/menu/update              : Update the existing item details of the menu

DELETE /api/v1/menu/delete              : Delete items from the Menu


                                        OTHER APIS

GET    /api/v1/api-doc                  : Get this page
GET    /health                          : Get server health
```

## Client
The client code is present in the directory client/.

The client is written using reqwest and tokio to do parallel http requests to the server asynchronously.

To execute the test:
```
Pre-requisite:
Make sure the server is running or else the client will fail.
Steps:
1. cd client
2. cargo build
3. export RUST_LOG=info
4. ./target/debug/client
```

```
./target/debug/client --help
client 0.1.0

USAGE:
    client [OPTIONS]

OPTIONS:
        --enable-dev-log-format <enable-dev-log-format>
            [env: ENABLE_DEV_LOG_FORMAT=] [default: true]

    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            Log level. dubug, info, warn, error [env: RUST_LOG=info] [default: info]

        --server-ip <SERVER_IP>
            [env: SERVER_IP=] [default: 127.0.0.1:8081]

    -V, --version
            Print version information
```


