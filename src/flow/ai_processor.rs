use crate::ai::ports::ai_i::AiI;
use crate::notification::adapters::discord::DiscordAdapter;
use log::info;

pub async fn ai_processor(
    ai_client: &impl AiI,
    discord_client: &DiscordAdapter,
    file_path: &str,
    file_name: &str,
) {
    info!("Starting AI processor");

    let response = ai_client.generate_resume(file_path, file_name).await;

    discord_client.send_message(response.as_str()).await
        .expect("Could not send message to Discord");
}
