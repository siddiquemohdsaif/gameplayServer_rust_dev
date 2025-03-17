mod firestore;
mod utils;
mod game_server;
mod game_data;
mod game_over;
mod game_event;

use actix_web::{web, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(game_server::new_connection))
            .route("/health_check", web::get().to(game_server::health_check))
    })
    .bind("0.0.0.0:14999")?
    .run()
    .await
}





