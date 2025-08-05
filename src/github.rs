use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CheckRun {
    pub name: String,
    pub head_sha: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub output: Option<CheckOutput>,
}

#[derive(Debug, Serialize)]
pub struct CheckOutput {
    pub title: String,
    pub summary: String,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckRunResponse {
    pub id: u64,
    pub name: String,
    pub status: String,
}

pub struct GitHubClient {
    client: Client,
    token: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    pub async fn create_check_run(
        &self,
        owner: &str,
        repo: &str,
        check_run: &CheckRun,
    ) -> Result<CheckRunResponse, reqwest::Error> {
        let url = format!("https://api.github.com/repos/{}/{}/check-runs", owner, repo);
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "smoked-tofu")
            .json(check_run)
            .send()
            .await?;

        response.json::<CheckRunResponse>().await
    }

    pub async fn update_check_run(
        &self,
        owner: &str,
        repo: &str,
        check_run_id: u64,
        check_run: &CheckRun,
    ) -> Result<CheckRunResponse, reqwest::Error> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/check-runs/{}",
            owner, repo, check_run_id
        );
        
        let response = self
            .client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "smoked-tofu")
            .json(check_run)
            .send()
            .await?;

        response.json::<CheckRunResponse>().await
    }
}