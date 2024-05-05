use actix_files as fs;
use actix_web::{App, HttpServer};
use std::env;
use std::path::PathBuf;
use lazy_static::lazy_static;

lazy_static! {
    static ref STATIC_DIR: PathBuf = {
        let exe_path = env::current_exe().unwrap();
        let bin_dir = exe_path.parent().unwrap();
        let result_dir = bin_dir.parent().unwrap();
        result_dir.join("static")
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", STATIC_DIR.clone().join("portfolio")).index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

