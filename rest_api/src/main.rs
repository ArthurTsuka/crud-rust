use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

mod controllers;
mod services;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DB_URL").expect("DB_URL environment variable not set");

    let (db_client, connection) = tokio_postgres::connect(&db_url, NoTls)
        .await
        .expect("Failed to connect to database");

    actix_web::rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let db_client_data = web::Data::new(db_client);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000") 
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .max_age(3600),  // Cache de CORS por 1 hora
            )
            .app_data(db_client_data.clone())
            .route("/product", web::post().to(controllers::product_controller::create_product))
            .route("/product", web::get().to(controllers::product_controller::get_all_products))
            .route("/product/{id}", web::get().to(controllers::product_controller::get_product_by_id))
            .route("/product", web::put().to(controllers::product_controller::update_product))
            .route("/product/{id}", web::delete().to(controllers::product_controller::delete_product))
            .route("/users", web::post().to(controllers::users_controller::create_user))
            .route("/users", web::get().to(controllers::users_controller::get_all_users))
            .route("/users/{id}", web::get().to(controllers::users_controller::get_user_by_id))
            .route("/users", web::put().to(controllers::users_controller::update_user))
            .route("/users/{id}", web::delete().to(controllers::users_controller::delete_user))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
