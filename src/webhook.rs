use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, error, warn};
use crate::app::AppState;
use crate::github::{CheckRun, CheckOutput};

#[derive(Debug, Deserialize)]
pub struct GitHubWebhookPayload {
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,
    pub commits: Option<Vec<Commit>>,
    pub repository: Repository,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub author: Author,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub message: String,
}

pub async fn handle_webhook(
    State(app_state): State<AppState>,
    Json(payload): Json<GitHubWebhookPayload>,
) -> Result<ResponseJson<WebhookResponse>, StatusCode> {
    info!("Received webhook for repository: {}", payload.repository.full_name);
    debug!("Webhook payload: {:?}", payload);
    
    if let Some(commits) = payload.commits {
        for commit in commits {
            info!("Processing commit: {} - {}", commit.id, commit.message);
            
            // Parse owner/repo from full_name
            let parts: Vec<&str> = payload.repository.full_name.split('/').collect();
            if parts.len() != 2 {
                warn!("Invalid repository full_name format: {}", payload.repository.full_name);
                continue;
            }
            let (owner, repo) = (parts[0], parts[1]);
            
            // Create initial check run
            let check_run = CheckRun {
                name: "smoked-tofu".to_string(),
                head_sha: commit.id.clone(),
                status: "in_progress".to_string(),
                conclusion: None,
                output: Some(CheckOutput {
                    title: "Running command".to_string(),
                    summary: "Executing configured command for this commit".to_string(),
                    text: None,
                }),
            };
            
            let check_run_response = match app_state.github_client.create_check_run(owner, repo, &check_run).await {
                Ok(response) => {
                    info!("Created check run {} for commit {}", response.id, commit.id);
                    response
                }
                Err(e) => {
                    error!("Failed to create check run for commit {}: {}", commit.id, e);
                    continue;
                }
            };
            
            // Execute the command
            let command_result = match app_state.command_executor.execute().await {
                Ok(result) => result,
                Err(e) => {
                    error!("Failed to execute command for commit {}: {}", commit.id, e);
                    continue;
                }
            };
            
            // Update check run with results
            let conclusion = if command_result.success { "success" } else { "failure" };
            let updated_check_run = CheckRun {
                name: "smoked-tofu".to_string(),
                head_sha: commit.id.clone(),
                status: "completed".to_string(),
                conclusion: Some(conclusion.to_string()),
                output: Some(CheckOutput {
                    title: format!("Command {}", if command_result.success { "succeeded" } else { "failed" }),
                    summary: command_result.summary(),
                    text: Some(format!("STDOUT:\n{}\n\nSTDERR:\n{}", command_result.stdout, command_result.stderr)),
                }),
            };
            
            match app_state.github_client.update_check_run(owner, repo, check_run_response.id, &updated_check_run).await {
                Ok(_) => {
                    info!("Updated check run {} for commit {} with result: {}", 
                         check_run_response.id, commit.id, conclusion);
                }
                Err(e) => {
                    error!("Failed to update check run {} for commit {}: {}", 
                          check_run_response.id, commit.id, e);
                }
            }
        }
    }
    
    Ok(ResponseJson(WebhookResponse {
        message: "Webhook processed successfully".to_string(),
    }))
}