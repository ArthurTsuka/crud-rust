use tokio_postgres::Client;
use crate::models::user::User;
use std::error::Error;

pub async fn create_user(body: &str, db_client: &Client) -> Result<(), Box<dyn Error>> {
    let user: User = serde_json::from_str(body)?;
    db_client.execute(
        "INSERT INTO users (name, email, number) VALUES ($1, $2, $3)",
        &[&user.name, &user.email, &user.number],
    ).await?;
    Ok(())
}

pub async fn get_all_users(db_client: &Client) -> Result<Vec<User>, Box<dyn Error>> {
    let rows = db_client.query("SELECT id, name, email, number FROM users", &[]).await?;
    let users: Vec<User> = rows.iter().map(|row| User {
        id: Some(row.get(0)),
        name: row.get(1),
        email: row.get(2),
        number: row.get(3),
    }).collect();
    Ok(users)
}

pub async fn get_user_by_id(user_id: &str, db_client: &Client) -> Result<User, Box<dyn Error>> {
    let id: i32 = user_id.parse()?;
    let row = db_client.query_one("SELECT id, name, email, number FROM users WHERE id = $1", &[&id]).await?;
    let user = User {
        id: Some(row.get(0)),
        name: row.get(1),
        email: row.get(2),
        number: row.get(3),
    };
    Ok(user)
}

pub async fn update_user(body: &str, db_client: &Client) -> Result<(), Box<dyn Error>> {
    let user: User = serde_json::from_str(body)?;
    db_client.execute(
        "UPDATE users SET name = $1, email = $2, number = $3 WHERE id = $4",
        &[&user.name, &user.email, &user.number, &user.id],
    ).await?;
    Ok(())
}

pub async fn delete_user(user_id: &str, db_client: &Client) -> Result<(), Box<dyn Error>> {
    let id: i32 = user_id.parse()?;
    db_client.execute("DELETE FROM users WHERE id = $1", &[&id]).await?;
    Ok(())
}
