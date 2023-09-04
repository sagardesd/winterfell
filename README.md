# winterfell
The api server is written in Rust using Axum crate and the server is using PostgreSql as Database.

Upon start the api server loads the Menu from the data/sample_menu.yaml file to the Database and creates the table Menu.

User application can view the menu by using "GET /api/v1/menu/view" api.

User application should then select only the item in the menu to place order for a table.
