use std::error::Error;

mod cache;
mod database;
mod entities;
mod namespace;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
