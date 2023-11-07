mod bot;
mod commands;
mod controllers;
mod err;
mod handlers;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    bot::run().await;
}
