mod firestore;
mod utils;
mod game_server;
mod game_data;
mod game_over;
mod game_event;
use serde::Deserialize;

use actix_web::{web, App, HttpServer};

const ENV_CONFIG_JSON: &str = include_str!("env_config.json");

#[derive(Debug, Deserialize)]
struct EnvConfig {
    #[serde(rename = "PRODUCTION_TYPE")]
    production_type: String
}

fn load_env_config() -> EnvConfig {
    serde_json::from_str::<EnvConfig>(ENV_CONFIG_JSON)
        .unwrap_or_else(|e| panic!("Invalid embedded JSON config: {}", e))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let env_cfg = load_env_config();

    let url = if env_cfg.production_type == "release" {
        "0.0.0.0:14999".to_string()
    }else{
        "0.0.0.0:15099".to_string()
    };

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(game_server::new_connection))
            .route("/health_check", web::get().to(game_server::health_check))
    })
    .bind(url)?
    .run()
    .await
}





