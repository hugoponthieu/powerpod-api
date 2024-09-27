use std::error::Error;

use database::Database;

mod database;
mod namespace;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(())
}
