use actix_files as fs;
use actix_web::{web, App, HttpServer};
use std::path::PathBuf;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    #[serde(flatten)]
    serve_configs: HashMap<String, ServeConfig>,
}

#[derive(Deserialize)]
struct ServeConfig {
    #[serde(rename = "hostAt")]
    serve_path: String,
}

lazy_static! {
    static ref CONFIG: Config = {
        let exe_path = std::env::current_exe().unwrap();
        let bin_dir = exe_path.parent().unwrap();
        let result_dir = bin_dir.parent().unwrap();
        let config_path = result_dir.join("lib").join("config.yaml");

        let config_str = std::fs::read_to_string(config_path).expect("Failed to read config file");
        serde_yaml::from_str(&config_str).expect("Failed to parse config file")
    };
    static ref STATIC_DIR: PathBuf = {
        let exe_path = std::env::current_exe().unwrap();
        let bin_dir = exe_path.parent().unwrap();
        let result_dir = bin_dir.parent().unwrap();
        result_dir.join("static")
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let mut app = App::new();

        for (category, serve_config) in CONFIG.serve_configs.iter() {
            let serve_dir = STATIC_DIR.join(category);
            let serve_path = serve_config.serve_path.clone();

            app = app.service(
                web::scope(&serve_path).service(fs::Files::new("/", serve_dir).index_file("index.html")),
            );
        }

        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
