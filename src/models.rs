use sqlx::prelude::FromRow;
use sqlx_model::{sanitize_name, SqliteModel};

use crate::AbcExportError;

#[derive(Debug, FromRow)]
pub struct Address {
    address_id: i64,
    country: String,
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, FromRow)]
pub struct Vendor {
    vendor_id: i64,
    code: String,
    name: String,
    purchaser_acct_number: Option<String>,
    email: Option<String>,
    salesperson: Option<String>,
    salesperson_phone: Option<String>,
    address_id: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct Customer {
    customer_id: i64,
    code: String,
    name: String,
    address_id: Option<i64>,
    phone: Option<String>,
    email: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct Item {
    item_id: i64,
    sku: String,
    secondary_skus: Option<String>,
    upcs: Option<String>,
    description: String,
    cost: i64,   // Costs go to 6 decimal places
    retail: i64, // Retails go to 2 decimal places
    primary_vendor_id: Option<i64>,
    stock_qty: i64, // Stock goes to 2 decimal places
    mtd_buy_qty: i64,
    mtd_sell_qty: i64,
    year_sell_qty: i64,
    year_buy_qty: i64,
}

impl Item {
    pub async fn select_by_upc(pool: &sqlx::SqlitePool, upc: &str) -> sqlx::Result<Vec<Self>> {
        let upc = format!("%{}%", upc);
        sqlx::query_as(&format!(
            "select * from {table_name}, json_each({table_name}.upcs) where json_each.value LIKE ?",
            table_name = Self::table_name()
        ))
        .bind(upc)
        .fetch_all(pool)
        .await
    }
}

impl SqliteModel for Address {
    type Error = AbcExportError;

    fn new() -> Self {
        Address {
            address_id: 0,
            country: "".to_string(),
            street: "".to_string(),
            city: "".to_string(),
            state: "".to_string(),
            zip: "".to_string(),
        }
    }
}

impl SqliteModel for Vendor {
    type Error = AbcExportError;

    fn new() -> Self {
        Vendor {
            vendor_id: 0,
            code: "".to_string(),
            name: "".to_string(),
            purchaser_acct_number: None,
            email: None,
            salesperson: None,
            salesperson_phone: None,
            address_id: None,
        }
    }
}

impl SqliteModel for Customer {
    type Error = AbcExportError;

    fn new() -> Self {
        Customer {
            customer_id: 0,
            code: "".to_string(),
            name: "".to_string(),
            address_id: None,
            phone: None,
            email: None,
        }
    }
}

impl SqliteModel for Item {
    type Error = AbcExportError;

    fn new() -> Self {
        Item {
            item_id: 0,
            sku: "".to_string(),
            secondary_skus: None,
            upcs: None,
            description: "".to_string(),
            cost: 0,   // Costs go to 6 decimal places
            retail: 0, // Retails go to 2 decimal places
            primary_vendor_id: None,
            stock_qty: 0, // Stock goes to 2 decimal places
            year_buy_qty: 0,
            year_sell_qty: 0,
            mtd_buy_qty: 0,
            mtd_sell_qty: 0,
        }
    }
}
