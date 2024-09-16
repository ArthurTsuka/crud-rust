pub mod users_controller;

use std::net::TcpStream;

pub fn handle_client(stream: TcpStream, db_url: &str) {
    users_controller::handle_user_requests(stream, db_url);
}
