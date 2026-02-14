use std::collections::HashMap;

use actix_web::{web, HttpResponse};

use crate::model::{ServiceDefinition, ServicesMap};
use crate::storage::AppState;

/// GET /api/services-map?meta.key=value&meta.env=prod
/// Returns full services map, optionally filtered by metadata query params.
/// Metadata filters use the `meta.` prefix: `?meta.team=payments&meta.env=prod`
pub async fn get_services_map(
    state: web::Data<AppState>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let filters: HashMap<String, String> = query
        .into_inner()
        .into_iter()
        .filter_map(|(k, v)| k.strip_prefix("meta.").map(|k| (k.to_string(), v)))
        .collect();
    let map = state.get_all(&filters);
    HttpResponse::Ok().json(map)
}

/// PUT /api/services-map
/// Replaces the entire services map.
pub async fn update_services_map(
    state: web::Data<AppState>,
    body: web::Json<ServicesMap>,
) -> HttpResponse {
    state.replace_all(body.into_inner());
    HttpResponse::Ok().json(state.get_all(&HashMap::new()))
}

/// GET /api/services/{name}
pub async fn get_service(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let name = path.into_inner();
    match state.get_by_name(&name) {
        Some(service) => HttpResponse::Ok().json(service),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Service '{}' not found", name)
        })),
    }
}

/// POST /api/services
pub async fn create_service(
    state: web::Data<AppState>,
    body: web::Json<ServiceDefinition>,
) -> HttpResponse {
    match state.create(body.into_inner()) {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({"status": "created"})),
        Err(e) => HttpResponse::Conflict().json(serde_json::json!({"error": e})),
    }
}

/// PUT /api/services/{name}
pub async fn update_service(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<ServiceDefinition>,
) -> HttpResponse {
    let name = path.into_inner();
    match state.update(&name, body.into_inner()) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({"status": "updated"})),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({"error": e})),
    }
}

/// DELETE /api/services/{name}
pub async fn delete_service(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let name = path.into_inner();
    match state.delete(&name) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({"status": "deleted"})),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({"error": e})),
    }
}
