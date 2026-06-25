use crate::ai::error::AiError;
use crate::ai::ports::ai_i::AiI;
use crate::config::AiProperties;
use crate::notification::ports::notification_i::NotificationI;
use crate::storage;
use crate::storage::create_file;
use crate::utils::file_format_utils::format_file_path;
use chrono::Local;
use log::info;
use std::path::PathBuf;

pub async fn ai_processor(
    ai_config: &AiProperties,
    ai_client: &impl AiI,
    discord_client: &impl NotificationI,
    files: &Vec<PathBuf>,
) {
    info!("Starting AI processor");

    if files.len() == 0 {
        info!("No files to process");
        return;
    }

    info!("{} files ", files.len());

    let mut responses: Vec<String> = Vec::new();

    for file in files.into_iter() {
        info!("Processing file {}", file.display());
        let response: Result<String, AiError> = ai_client
            .generate_resume(
                file,
                &ai_config.article_resume_system_prompt_path,
                &ai_config.user_prompt_path,
            )
            .await;

        match response {
            Ok(resp) => {
                responses.push(resp);
            }
            Err(err) => {
                let mut message_truncate = err.message.clone();
                message_truncate.truncate(500);
                message_truncate.push_str("...");
                discord_client
                    .send_message(message_truncate.as_str())
                    .await
                    .expect("Could not send message to Discord");
                panic!("AI processor error: {}", err)
            }
        }
    }

    info!("{} responses are processed", responses.len());

    let resume_path = "ai_resume";
    storage::check_or_create_folder(&resume_path)
        .expect("Unable to check or create folder ai_resume");

    let mut content_response: String = String::new();

    for resp in responses.into_iter() {
        content_response += "\n--------\n";
        content_response += &resp;
    }

    let global_resume_path: String = format_file_path(resume_path, "resume", &Local::now().date_naive());
    create_file(global_resume_path.as_str(), &content_response);

    info!("Processing general resume");
    let global_resume = ai_client
        .generate_resume(
            &PathBuf::from(global_resume_path),
            &ai_config.global_resume_system_prompt_path,
            &ai_config.user_prompt_path,
        )
        .await;

    match global_resume {
        Ok(global_resume) => {
            discord_client
                .send_message(global_resume.as_str())
                .await
                .expect("Could not send message to Discord");
        }
        Err(err) => {
            let mut message_truncate = err.message.clone();
            message_truncate.truncate(500);
            message_truncate.push_str("...");
            discord_client
                .send_message(message_truncate.as_str())
                .await
                .expect("Could not send message to Discord");
            panic!("AI processor error: {}", err)
        }
    }
}
