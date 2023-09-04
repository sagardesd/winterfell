use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderStatus {
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetItemResponse {
    pub table_no: u32,
    pub name: String,
    pub quantity: u32,
    pub preparation_time: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Menu {
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuItem {
    pub name: String,
    pub description: String,
    pub price: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuItemUpdate {
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableBill {
    pub table_no: u32,
    pub total_bill: f64,
}
