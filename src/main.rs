use std::error::Error;

use database::Database;

mod database;
mod namespace;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pgdatabase: Database =
        match Database::new("postgres://postgres:postgres@localhost:5432/powerpod").await {
            Ok(database) => database,
            Err(e) => return Err(e),
        };
    println!("Hello, world!");
    Ok(())
}
