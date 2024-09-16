use crate::models::user::User;
use postgres::{Client, NoTls};
use crate::utils::http_responses::{OK_RESPONSE, NOT_FOUND, INTERNAL_SERVER_ERROR};

pub fn create_user(request: &str, db_url: &str) -> (String, String) {

    match (parse_user_request(request), Client::connect(db_url, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            println!("Conexão bem-sucedida, tentando criar usuário...");

            match client.execute(
                "INSERT INTO users (name, email, number) VALUES ($1, $2, $3)",
                &[&user.name, &user.email, &user.number]
            ) {
                Ok(_) => (OK_RESPONSE.to_string(), "User Created".to_string()),
                Err(e) => {
                    println!("Erro ao criar o usuário no banco de dados: {}", e);
                    (INTERNAL_SERVER_ERROR.to_string(), "Error creating user".to_string())
                }
            }
        },
        (Err(e), _) => {
            println!("Erro ao parsear o usuário: {}", e);
            (INTERNAL_SERVER_ERROR.to_string(), "Error parsing user".to_string())
        },
        (_, Err(e)) => {
            println!("Erro ao conectar ao banco de dados: {}", e);
            (INTERNAL_SERVER_ERROR.to_string(), "Database connection error".to_string())
        }
    }
}

pub fn get_user_by_id(request: &str, db_url: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(db_url, NoTls)) {
        (Ok(id), Ok(mut client)) => match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
            Ok(row) => {
                let user = User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    number: row.get(3),
                };

                (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
            }
            _ => (NOT_FOUND.to_string(), "User not Found".to_string()),
        },

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn get_all_users(request: &str, db_url: &str) -> (String, String) {
    let _ = request;
    match Client::connect(db_url, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();

            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    number: row.get(3),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn update_user(request: &str, db_url: &str) -> (String, String) {
    match(get_id(&request).parse::<i32>(), parse_user_request(&request), Client::connect(db_url, NoTls)) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client.execute(
                "UPDATE users SET name = $1, email = $2, number = $3 WHERE id = $4",
                &[&user.name, &user.email, &user.number, &id]
            ).unwrap();

        (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn delete_user(request: &str, db_url: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(db_url, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_afetadas = client.execute(
                "DELETE FROM users WHERE id = $1",
                &[&id]
            ).unwrap();

            if rows_afetadas == 0 {
                return (NOT_FOUND.to_string(), "User not Found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

pub fn parse_user_request(request: &str) -> Result<User, serde_json::Error> {
    let body = request.split("\r\n\r\n").last().unwrap_or_default().trim();
    serde_json::from_str(body)
}


