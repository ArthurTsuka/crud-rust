mod controllers; // Se você tiver um módulo de controladores
mod db;          // Se você tiver um módulo de banco de dados
mod models;      // Para o módulo models
mod services;    // Para o módulo services
mod utils;       // Para o módulo utils

use std::net::TcpListener;



fn main() {
    let db_url = std::env::var("DB_URL").expect("DB_URL environment variable not set");

    // Configura o banco de dados
    if let Err(e) = db::set_database(&db_url) {
        println!("Error: {}", e);
        return;
    }

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server started at port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => controllers::handle_client(stream, &db_url),
            Err(e) => println!("Error: {}", e),
        }
    }
}
