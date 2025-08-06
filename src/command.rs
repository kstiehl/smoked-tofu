use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "smoked-tofu")]
#[command(about = "A GitHub webhook server that runs commands on commits using the Checks API")]
pub struct Args {
    #[arg(short, long, help = "GitHub personal access token")]
    pub token: String,
    
    #[arg(short, long, help = "GitHub webhook secret for signature verification")]
    pub secret: String,
    
    #[arg(short, long, default_value = "3000", help = "Port to run the webhook server on")]
    pub port: u16,
    
    #[arg(short, long, help = "Command to run when commits are received")]
    pub command: String,
    
    #[arg(help = "Arguments to pass to the command")]
    pub args: Vec<String>,
}

pub struct CommandExecutor {
    command: String,
    args: Vec<String>,
}

impl CommandExecutor {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }

    pub async fn execute(&self) -> Result<CommandResult, std::io::Error> {
        let output = Command::new(&self.command)
            .args(&self.args)
            .output()?;

        Ok(CommandResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
        })
    }
}

#[derive(Debug)]
pub struct CommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

impl CommandResult {
    pub fn summary(&self) -> String {
        if self.success {
            format!("Command succeeded (exit code: {:?})", self.exit_code)
        } else {
            format!("Command failed (exit code: {:?})", self.exit_code)
        }
    }
}