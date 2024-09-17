use actix_web::{web, HttpResponse, Responder};
use crate::services::product_service;
use tokio_postgres::Client;

pub async fn create_product(db_client: web::Data<Client>, body: String) -> impl Responder {
    match product_service::create_product(&body, db_client.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Product created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create product: {}", e)),
    }
}

pub async fn get_all_products(db_client: web::Data<Client>) -> impl Responder {
    match product_service::get_all_products(db_client.get_ref()).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch products: {}", e)),
    }
}

pub async fn get_product_by_id(db_client: web::Data<Client>, product_id: web::Path<String>) -> impl Responder {
    match product_service::get_product_by_id(&product_id, db_client.get_ref()).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => HttpResponse::NotFound().body(format!("Product not found: {}", e)),
    }
}

pub async fn update_product(db_client: web::Data<Client>, body: String) -> impl Responder {
    match product_service::update_product(&body, db_client.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Product updated successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update product: {}", e)),
    }
}

pub async fn delete_product(db_client: web::Data<Client>, product_id: web::Path<String>) -> impl Responder {
    match product_service::delete_product(&product_id, db_client.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Product deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete product: {}", e)),
    }
}


