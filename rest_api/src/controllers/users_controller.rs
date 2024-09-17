use actix_web::{web, HttpResponse, Responder};
use crate::services::user_service;
use tokio_postgres::Client;

// Handler para criar usuário
pub async fn create_user(db_client: web::Data<Client>, body: String) -> impl Responder {
    match user_service::create_user(&body, db_client.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create user: {}", e)),
    }
}

// Handler para pegar todos os usuários
pub async fn get_all_users(db_client: web::Data<Client>) -> impl Responder {
    match user_service::get_all_users(db_client.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch users: {}", e)),
    }
}

// Handler para pegar usuário por ID
pub async fn get_user_by_id(db_client: web::Data<Client>, user_id: web::Path<String>) -> impl Responder {
    match user_service::get_user_by_id(&user_id, db_client.get_ref()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(format!("User not found: {}", e)),
    }
}

// Handler para atualizar usuário
pub async fn update_user(db_client: web::Data<Client>, body: String) -> impl Responder {
    match user_service::update_user(&body, db_client.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("User updated successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update user: {}", e)),
    }
}

// Handler para deletar usuário por ID
pub async fn delete_user(db_client: web::Data<Client>, user_id: web::Path<String>) -> impl Responder {
    match user_service::delete_user(&user_id, db_client.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("User deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete user: {}", e)),
    }
}
