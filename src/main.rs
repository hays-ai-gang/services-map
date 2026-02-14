mod handlers;
mod model;
mod storage;

use actix_web::{web, App, HttpServer};
use storage::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data_file = std::env::var("DATA_FILE").unwrap_or_else(|_| "services-map.json".to_string());
    let state = web::Data::new(AppState::new(data_file));

    println!("Starting services-map on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // Full service map
            .route(
                "/api/services-map",
                web::get().to(handlers::get_services_map),
            )
            .route(
                "/api/services-map",
                web::put().to(handlers::update_services_map),
            )
            // Single service CRUD
            .route("/api/services", web::post().to(handlers::create_service))
            .route("/api/services/{name}", web::get().to(handlers::get_service))
            .route(
                "/api/services/{name}",
                web::put().to(handlers::update_service),
            )
            .route(
                "/api/services/{name}",
                web::delete().to(handlers::delete_service),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
