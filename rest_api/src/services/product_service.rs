use tokio_postgres::Client;
use crate::models::product::Product;
use std::error::Error;

pub async fn create_product(body: &str, db_client: &Client) -> Result<(), Box<dyn Error>> {
    let product: Product = serde_json::from_str(body)?;
    db_client.execute(
        "INSERT INTO products (name, category, price, description, in_stock, weight, discount) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&product.name, &product.category, &product.price, &product.description, &product.in_stock, &product.weight, &product.discount],
    ).await?;
    Ok(())
}

pub async fn get_all_products(db_client: &Client) -> Result<Vec<Product>, Box<dyn Error>> {
    let rows = db_client.query("SELECT id, name, category, price, description, in_stock, weight, discount FROM products", &[]).await?;
    let products: Vec<Product> = rows.iter().map(|row| Product {
        id: Some(row.get(0)),
        name: row.get(1),
        category: row.get(2),
        price: row.get(3),
        description: row.get(4),
        in_stock: row.get(5),
        weight: row.get(6),
        discount: row.get(7),
    }).collect();
    Ok(products)
}

pub async fn get_product_by_id(product_id: &str, db_client: &Client) -> Result<Product, Box<dyn Error>> {
    let id: i32 = product_id.parse()?;
    let row = db_client.query_one("SELECT id, name, category, price, description, in_stock, weight, discount FROM products WHERE id = $1", &[&id]).await?;
    let product = Product {
        id: Some(row.get(0)),
        name: row.get(1),
        category: row.get(2),
        price: row.get(3),
        description: row.get(4),
        in_stock: row.get(5),
        weight: row.get(6),
        discount: row.get(7),
    };
    Ok(product)
}

pub async fn update_product(body: &str, db_client: &Client) -> Result<(), Box<dyn Error>> {
    let product: Product = serde_json::from_str(body)?;
    db_client.execute(
        "UPDATE products SET name = $1, category = $2, price = $3, description = $4, in_stock = $5, weight = $6, discount = $7 WHERE id = $8",
        &[&product.name, &product.category, &product.price, &product.description, &product.in_stock, &product.weight, &product.discount, &product.id],
    ).await?;
    Ok(())
}

pub async fn delete_product(product_id: &str, db_client: &Client) -> Result<(), Box<dyn Error>> {
    let id: i32 = product_id.parse()?;
    db_client.execute("DELETE FROM products WHERE id = $1", &[&id]).await?;
    Ok(())
}


