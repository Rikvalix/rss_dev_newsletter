use crate::config::GlobalProperties;
use crate::domain::database::model::RepositoryRepoHandler;

pub struct ApplicationConfiguration {
    pub repositories: RepositoryRepoHandler,
    pub config: GlobalProperties
}

impl ApplicationConfiguration {
    pub fn new(
        repository_repo_handler: RepositoryRepoHandler,
        config: GlobalProperties
    ) -> ApplicationConfiguration {
        ApplicationConfiguration {
            repositories: repository_repo_handler,
            config
        }
    }
}
