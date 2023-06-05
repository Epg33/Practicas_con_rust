pub mod db {
    extern crate mongodb;
    use mongodb::{bson::doc, Client, options::ClientOptions};
    pub async fn connect_db() {
        let user = std::env::var("USERNAME").ok();
        let pass = std::env::var("PASSWORD").ok();
        let connection_string = format!("mongodb+srv://{}:{}@cluster0.tcpeflz.mongodb.net/?retryWrites=true&w=majority", user.unwrap(), pass.unwrap());
        let client_options = ClientOptions::parse(connection_string).await.unwrap();
        let client = Client::with_options(client_options).expect("a message bro");
        println!("connect to db");
    }
}
