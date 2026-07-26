use crate::domain::application::model::ApplicationConfiguration;
use crate::domain::web::model::{HealthDto, MetadataDto, NewsletterDto, NewsletterShortDto};
use crate::infrastructure::database::repository::summary_repository::SummaryRepository;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

#[derive(Clone)]
pub struct WebHandler {
    pub summary_repository: SummaryRepository,
}

impl WebHandler {
    pub fn new(application_configuration: &ApplicationConfiguration) -> WebHandler {
        WebHandler {
            summary_repository: application_configuration
                .repositories
                .summary_repository
                .clone(),
        }
    }
    pub async fn health_status(State(state): State<Self>) -> (StatusCode, Json<HealthDto>) {
        let dto= HealthDto{active:true};

        (StatusCode::OK,Json(dto))
    }

    // Newsletter

    pub async fn latest_newsletter(State(state): State<Self>) -> (StatusCode, Json<NewsletterDto>) {
        let latest_summary = state.summary_repository.find_latest().await.unwrap();

        let dto = NewsletterDto {
            public_id: latest_summary.public_id.to_string(),
            date: latest_summary.created_at.date_naive(),
            title: latest_summary.title.to_string(),
            content: latest_summary.content.to_string(),
            metadata: MetadataDto {
                model: Some(latest_summary.metadata.model.clone().unwrap().to_string()),
            },
        };

        (StatusCode::OK, Json(dto))
    }

    pub async fn page_newsletter(&self) -> (StatusCode, Json<NewsletterShortDto>) {
        todo!()
    }
}
