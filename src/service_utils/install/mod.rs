#![allow(unused)]
mod linux;
mod wind_mac;
use crate::Result;
use linux::*;
pub use reqwest::Response;

use std::path::{Path, PathBuf};
use wind_mac::*;

/// unified interface for installing ollama tool, ***password*** parameter is exclusive for linux !!
#[cfg(target_os = "linux")]
pub async fn install_tool(password: &str) -> Result<()> {
    install_linux_tool(password).await?;

    Ok(())
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub async fn install_tool<F, Fut>(f_stream: F) -> Result<()>
where
    F: FnOnce(Response, PathBuf) -> Fut,
    Fut: Future<Output = Result<PathBuf>>,
{
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    install_windows_arm_tool(f_stream).await?;

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    install_windows_x86_tool(f_stream).await?;

    #[cfg(target_os = "macos")]
    install_macos_tool(f_stream).await?;
}
