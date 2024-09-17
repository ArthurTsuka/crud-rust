use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: Option<i32>,
    pub name: String,
    pub category: String,
    pub price: i32,
    pub description: String,
    pub in_stock: bool,
    pub weight: i32,
    pub discount: Option<i32>,
}   