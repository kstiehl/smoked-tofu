use crate::command::{CommandExecutor, Args};
use crate::github::GitHubClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub github_client: Arc<GitHubClient>,
    pub command_executor: Arc<CommandExecutor>,
    pub webhook_secret: String,
}

impl AppState {
    pub fn new(args: Args) -> Self {
        let github_client = Arc::new(GitHubClient::new(args.token));
        let command_executor = Arc::new(CommandExecutor::new(args.command, args.args));
        
        Self {
            github_client,
            command_executor,
            webhook_secret: args.secret,
        }
    }
}