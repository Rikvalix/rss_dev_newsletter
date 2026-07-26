use crate::application::process::ai_processor::AiProcessor;
use crate::application::process::rss_processor::RssProcessor;
use crate::domain::database::model::RepositoryRepoHandler;

pub struct ApplicationConfiguration {
    pub repositories: RepositoryRepoHandler,
    pub rss_processor: RssProcessor,
    pub ai_processor: AiProcessor,
}

impl ApplicationConfiguration {
    pub fn new(
        repository_repo_handler: RepositoryRepoHandler,
        rss_processor: RssProcessor,
        ai_processor: AiProcessor,
    ) -> ApplicationConfiguration {
        ApplicationConfiguration {
            repositories: repository_repo_handler,
            rss_processor,
            ai_processor,
        }
    }
}
