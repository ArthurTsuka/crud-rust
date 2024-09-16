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

    Ok(())
}