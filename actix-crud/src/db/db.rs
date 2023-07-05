pub mod db {
    extern crate mongodb;
    use mongodb::{bson::doc, Client, options::{ClientOptions, ServerApi, ServerApiVersion}};
    pub async fn connect_db() -> mongodb::error::Result<()>{
        let user = std::env::var("USERNAME").ok();
        let pass = std::env::var("PASSWORD").ok();
        let connection_string = format!("mongodb+srv://{}:{}@actix-crud.ezlbbp1.mongodb.net/?retryWrites=true&w=majority", user.unwrap(), pass.unwrap());
        let mut client_options = ClientOptions::parse(connection_string).await.unwrap();
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options).expect("nou");
        client.database("admin").run_command(doc! {"ping": 1}, None).await?;
        println!("connected to db");
        Ok(())
    }
}
