mod linux;
mod wind_mac;

use crate::OResult;
use linux::*;

use reqwest::Response;
use std::path::{Path, PathBuf};
use wind_mac::*;

/// unified interface for installing ollama tool, ***password*** parameter is exclusive for linux !!
pub async fn install_tool(
    #[cfg(target_os = "linux")] password: &str,
    #[cfg(any(target_os = "windows", target_os = "macos"))] f_stream: impl AsyncFnOnce(
        Response,
        &mut Path,
    ) -> OResult<
        PathBuf,
    >,
) -> OResult<()> {
    #[cfg(target_os = "linux")]
    install_linux_tool(password).await?;

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    install_windows_arm_tool(f_stream).await?;

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    install_windows_x86_tool(f_stream).await?;

    #[cfg(target_os = "macos")]
    install_macos_tool(f_stream).await?;
    Ok(())
}
