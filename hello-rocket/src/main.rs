#[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};
use std::io;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

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
        .mount("/", routes![blocking_task])
        .mount("/", routes![files])
}