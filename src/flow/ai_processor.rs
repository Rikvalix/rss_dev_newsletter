use crate::ai::error::AiError;
use crate::ai::ports::ai_i::AiI;
use crate::notification::adapters::discord::DiscordAdapter;
use log::info;
use std::path::PathBuf;

pub async fn ai_processor(
    ai_client: &impl AiI,
    discord_client: &DiscordAdapter,
    files: &Vec<PathBuf>,
) {
    info!("Starting AI processor");
    info!("{} files ", files.len());

    //let mut responses: Vec<String> = Vec::new();

    for file in files.into_iter() {
        //responses.push(ai_client.generate_resume(file).await);
        let response: Result<String, AiError> = ai_client.generate_resume(file).await;
        match response {
            Ok(resp) => {
                discord_client
                    .send_message(resp.as_str())
                    .await
                    .expect("Could not send message to Discord");
            }
            Err(err) => {
                discord_client
                    .send_message(err.message.as_str())
                    .await
                    .expect("Could not send message to Discord");
                panic!("AI processor error: {}", err)
            }
        }
    }
}
