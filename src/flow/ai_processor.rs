use crate::ai::error::AiError;
use crate::ai::ports::ai_i::AiI;
use crate::config::AiProperties;
use crate::notification::ports::notification_i::NotificationI;
use crate::storage;
use crate::storage::create_file;
use crate::utils::file_format_utils::format_file_path;
use chrono::Local;
use log::{error, info};
use std::path::PathBuf;

/// Generate resume for each emails and last one global resume which be send.
///
/// # Arguments
///
/// * `ai_config`: Ai configuration
/// * `ai_client`: Ai client, use to generate resume
/// * `notification_client`: Notification client, share status and resume
/// * `files`: List of paths to analyse
///
/// returns: ()
pub async fn ai_processor(
    ai_config: &AiProperties,
    ai_client: &impl AiI,
    notification_client: &impl NotificationI,
    files: &Vec<PathBuf>,
) -> Result<(), AiError> {
    info!("Starting AI processor");

    if files.len() == 0 {
        info!("No files to process");
    }

    info!("{} files ", files.len());

    let mut responses: Vec<String> = Vec::new();

    for file in files.into_iter() {
        info!("Processing file {}", file.display());
        let response: String = ai_client
            .generate_resume(
                file,
                &ai_config.article_resume_system_prompt_path,
                &ai_config.user_prompt_path,
            )
            .await
            .map_err(|err| {
                error!("Error while processing file {}: {}", file.display(), err);
                err
            })?;

        responses.push(response);
    }

    info!("{} responses are processed", responses.len());

    let resume_path = "ai_resume";
    storage::check_or_create_folder(&resume_path)?;

    let mut content_response: String = String::new();

    for resp in responses.into_iter() {
        content_response += "\n--------\n";
        content_response += &resp;
    }

    let global_resume_path: PathBuf = create_file(
        format_file_path(resume_path, "resume", &Local::now().date_naive()).as_str(),
        &content_response,
    )?;

    info!("Processing general resume");
    let global_resume = ai_client
        .generate_resume(
            &PathBuf::from(global_resume_path),
            &ai_config.global_resume_system_prompt_path,
            &ai_config.user_prompt_path,
        )
        .await
        .map_err(|err| {
            error!("Error while generating resume: {}", err);
            err
        })?;

    notification_client
        .send_message(global_resume.as_str())
        .await?;

    Ok(())
}
