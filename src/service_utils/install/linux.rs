use std::error::Error;
use std::fmt::Display;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use crate::OResult;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct WrongPass(pub String);

impl Display for WrongPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for WrongPass {}

async fn check_password(password: &str) -> OResult<bool> {
    let mut sudo = Command::new("sudo")
        .args(["-S", "-k", "true"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    if let Some(mut stdin) = sudo.stdin.take() {
        if stdin
            .write_all(format!("{}\n", password).as_bytes())
            .await
            .is_err()
        {
            return Ok(false);
        }
    }
    Ok(sudo
        .wait()
        .await
        .map(|status| status.success())
        .unwrap_or(false))
}

pub(super) async fn install_linux_tool(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !check_password(password).await? {
        return Err(Box::new(WrongPass(format!(
            "wrong password : {}",
            password
        ))));
    }

    let mut sudo = Command::new("sudo")
        .arg("-S")
        .args(["sh", "-c", "curl -fsSL https://ollama.com/install.sh | sh"])
        .stdin(Stdio::piped()) // Allows writing password
        .stdout(Stdio::inherit()) // Display output in terminal
        .stderr(Stdio::inherit()) // Display errors in terminal
        .spawn()?;

    if let Some(mut stdin) = sudo.stdin.take() {
        // Write password to sudo's stdin
        stdin
            .write_all(format!("{}\n", password).as_bytes())
            .await?;
    }

    sudo.wait().await?;
    Ok(())
}

#[cfg(test)]
mod quick_test {
    use super::{check_password, install_linux_tool};

    #[tokio::test]
    async fn install_tool_correct_pass() {
        let tool = install_linux_tool("0788").await;
        assert!(tool.is_ok());
    }
    #[tokio::test]
    async fn install_tool_wrong_pass() {
        let tool = install_linux_tool("07894").await;
        assert!(tool.is_err());
    }
    #[tokio::test]
    async fn check_pass() {
        let wrong = check_password("07987").await.unwrap();
        let correct = check_password("0788").await.unwrap();

        assert!(correct);
        assert!(!wrong);
    }
}
