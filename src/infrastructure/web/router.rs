use crate::domain::application::model::ApplicationConfiguration;
use crate::infrastructure::web::handler::WebHandler;
use axum::routing::get;
use axum::Router;

pub fn get_router(application_configuration: &ApplicationConfiguration) -> Router {
    let handler = WebHandler::new(&application_configuration);
    Router::new()
        .route("/health", get(WebHandler::health_status))
        .route("/api/v1/newsletter/latest",get(WebHandler::get_latest_newsletter))
        .route("/api/v1/newsletter/all",get(WebHandler::all_shorts))
        .route("/api/v1/newsletter/{public_id}",get(WebHandler::get_by_public_id))
        .with_state(handler)
}