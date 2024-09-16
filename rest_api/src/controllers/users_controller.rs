use crate::services::user_service;
use crate::utils::http_responses::NOT_FOUND;
use std::net::TcpStream;
use std::io::{Read, Write};

pub fn handle_user_requests(mut stream: TcpStream, db_url: &str) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => user_service::create_user(r, db_url),
                r if r.starts_with("GET /users/") => user_service::get_user_by_id(r, db_url),
                r if r.starts_with("PUT /users") => user_service::update_user(r, db_url),
                r if r.starts_with("GET /users") => user_service::get_all_users(r, db_url),
                r if r.starts_with("DELETE /users/") => user_service::delete_user(r, db_url),
                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
