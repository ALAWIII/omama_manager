use std::env::current_exe;
use std::path::PathBuf;

mod check;
mod install;
mod models_manager;
use crate::Result;
pub use check::*;
pub use install::install_tool;
pub use models_manager::*;

/// get current executable path.
pub fn get_current_path() -> Result<PathBuf> {
    Ok(current_exe()?.parent().unwrap().to_path_buf())
}
