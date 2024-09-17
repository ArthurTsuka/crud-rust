use postgres::{Client, NoTls, Error};

pub fn set_database(db_url: &str) -> Result<(), Error> {
    let mut client = Client::connect(db_url, NoTls)?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL,
            number INTEGER NOT NULL
        )",
        &[]
    )?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            category VARCHAR NOT NULL,
            price INTEGER NOT NULL,
            description TEXT NOT NULL,
            in_stock BOOLEAN NOT NULL,
            weight INTEGER NOT NULL,
            discount INTEGER
        )",
        &[]
    )?;

    Ok(())
}