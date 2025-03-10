use super::get_current_path;
use super::EXECUTABLE;
use crate::OResult;
use std::process::Command;

/// only used to check whether ollama exists in the same Omama executable directory , only for windows and macos , linux must use the official script to setup everything including the path!!!
pub fn is_installed_locally() -> OResult<bool> {
    let current_path = get_current_path()?.join(EXECUTABLE);
    Ok(current_path.exists())
}
/// check whether ollama exists in the system path, if not then this means that its not installed globally!!
pub fn is_installed_globally() -> bool {
    const COMMAND: &str = if cfg!(windows) { "where" } else { "which" };

    Command::new(COMMAND)
        .arg("ollama")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod quick_test {
    use crate::OResult;

    use super::{is_installed_globally, is_installed_locally};

    #[test]
    fn local_ollama_path() -> OResult<()> {
        let local = is_installed_locally()?;
        println!("locally: {}", local);
        Ok(())
    }
    #[test]
    fn global_ollama_path() {
        let global = is_installed_globally();
        println!("globally: {}", global)
    }
}
