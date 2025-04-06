use super::get_current_path;
use crate::Result;
use reqwest::get;
use tokio::process::Command;

pub(super) const EXECUTABLE: &str = if cfg!(windows) {
    "ollama-windows/ollama.exe"
} else {
    "Ollama.app/Contents/MacOS/Ollama"
};

/// test whether the ollama is actually running in the background.
pub async fn is_ollama_running() -> bool {
    get("http://localhost:11434").await.is_ok()
}

/// attempts to start the services if ollama is installed globally , otherwise attempts locally.
pub async fn start_ollama_service(globally: bool) -> Result<()> {
    let tool = if globally {
        "ollama".to_string()
    } else {
        get_current_path()?
            .join(EXECUTABLE)
            .to_string_lossy()
            .to_string()
    };
    let mut command = Command::new(tool);
    command.arg("serve");

    command.spawn()?;
    Ok(())
}
