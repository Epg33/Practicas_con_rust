mod routes;
mod db;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    routes::routes::routes::hello();
    connect().await;
    println!("Hello, world!");
}

async fn connect() {
    db::db::db::connect_db().await;
}
