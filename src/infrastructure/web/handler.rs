use crate::domain::application::model::ApplicationConfiguration;
use crate::domain::database::model::SummaryEntity;
use crate::domain::web::model::{HealthDto, MetadataDto, NewsletterDto, NewsletterShortDto};
use crate::infrastructure::database::repository::summary_repository::SummaryRepository;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

#[derive(Debug,Clone)]
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
    pub async fn health_status() -> (StatusCode, Json<HealthDto>) {
        let dto= HealthDto{active:true};

        (StatusCode::OK,Json(dto))
    }

    // Newsletter

    pub async fn get_by_public_id(
        Path(public_id): Path<String>,
        State(state): State<Self>) -> (StatusCode, Json<Option<NewsletterDto>>) {
        let entity_opt = state.summary_repository.find_by_public_id(public_id).await.unwrap();

        if entity_opt.is_none() {
            return (StatusCode::NO_CONTENT, Json(None));
        } 

        let entity = entity_opt.unwrap();

        let dto = NewsletterDto {
            public_id: entity.public_id.to_string(),
            date: entity.created_at.date_naive(),
            title: entity.title.to_string(),
            content: entity.content.to_string(),
            metadata: MetadataDto {
                model: Some(entity.metadata.model.clone().unwrap().to_string()),
            },
        };

        (StatusCode::OK,Json(Some(dto)))
    }

    pub async fn get_latest_newsletter(State(state): State<Self>) -> (StatusCode, Json<Option<NewsletterDto>>) {
        let latest_summary_opt: Option<SummaryEntity> = state.summary_repository.find_latest().await.unwrap();

        if latest_summary_opt.is_none() {
           return (StatusCode::NO_CONTENT, Json(None));
        }
        let entity = latest_summary_opt.unwrap();

        let dto = NewsletterDto {
            public_id: entity.public_id.to_string(),
            date: entity.created_at.date_naive(),
            title: entity.title.to_string(),
            content: entity.content.to_string(),
            metadata: MetadataDto {
                model: Some(entity.metadata.model.clone().unwrap().to_string()),
            },
        };

        (StatusCode::OK, Json(Some(dto)))
    }

    pub async fn all_shorts(State(state) : State<Self>) -> (StatusCode, Json<Vec<NewsletterShortDto>>) {
        let summaries: Vec<crate::domain::database::model::SummaryEntity> = state.summary_repository.find_all().await.unwrap();

        let dtos: Vec<NewsletterShortDto> = summaries.iter()
        .map(|it| NewsletterShortDto
             { 
                public_id: it.public_id.to_string(), 
                date: it.created_at.date_naive(), 
                title: it.title.clone() 
            }

        )
        .collect();

        (StatusCode::OK, Json(dtos))
    }
}
