use crate::config::AiProperties;
use crate::infrastructure::ai::error::AiError;
use crate::ports::ai_i::AiI;
use crate::ports::notification_i::NotificationI;
use crate::storage;
use crate::storage::create_file;
use crate::utils::file_format_utils::format_file_path;
use chrono::Local;
use log::{error, info};
use std::path::PathBuf;

/// Generate summary for each emails and last one global summary which be send.
///
/// # Arguments
///
/// * `ai_config`: Ai configuration
/// * `ai_client`: Ai client, use to generate summary
/// * `notification_client`: Notification client, share status and summary
/// * `files`: List of paths to analyse
///
/// returns: ()
pub async fn ai_processor(
    ai_config: &AiProperties,
    ai_client: &impl AiI,
    notification_client: &impl NotificationI,
    files: &[PathBuf],
) -> Result<(), AiError> {
    info!("Starting AI processor");

    if files.is_empty() {
        info!("No files to process");
    }

    info!("{} files ", files.len());

    let mut responses: Vec<String> = Vec::new();

    let temporary_summary_path = "ai_summary/temporary";
    storage::check_or_create_folder(temporary_summary_path)?;

    for file in files.iter() {
        info!("Processing file {}", file.display());
        let response: String = ai_client.generate_summary(
                file,
                &ai_config.article_summary_system_prompt_path,
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



    info!("Processing general summary");
    let global_summary = ai_client
        .generate_summary(
            &PathBuf::from(format_file_path(temporary_summary_path, "temp_summary", &Local::now().date_naive()).as_str()),
            &ai_config.global_summary_system_prompt_path,
            &ai_config.user_prompt_path,
        )
        .await
        .map_err(|err| {
            error!("Error while generating summary: {}", err);
            err
        })?;

    let global_summary_path = "ai_summary/global";
    storage::check_or_create_folder(global_summary_path)?;

    create_file(
        format_file_path(global_summary_path, "summary", &Local::now().date_naive()).as_str(),
        &global_summary,
    )?;

    notification_client
        .send_message(global_summary.as_str())
        .await?;

    Ok(())
}
