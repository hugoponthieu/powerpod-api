use std::error::Error;


mod cluster;
mod services;
mod repositories;
mod cache;
mod database;
mod entities;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
