use crate::ai::ports::ai_i::AiI;
use log::info;

pub async fn ai_processor(client: &impl AiI, file_path: &str, file_name: &str) {
    info!("Starting AI processor");

    client.generate_resume(file_path, file_name).await;

}