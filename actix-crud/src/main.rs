mod routes;
mod db;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    routes::routes::routes::hello();
    connect();
    println!("Hello, world!");
}

async fn connect() {
    db::db::db::connect_db().await;
}
