#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    return format!("waited for {seconds} seconds");
}

#[get("/home/<name>")]
fn name (name: String) -> String {
    return format!("Hello {name}");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![name])
        .mount("/", routes![delay])
}