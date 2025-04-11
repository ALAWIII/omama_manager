pub use ollama_rs::models::pull::PullModelStatusStream;
use once_cell::sync::Lazy;
pub mod service_utils;
pub use anyhow::{Result, anyhow};

mod asset;
pub mod chat;
pub mod database;
pub mod rag;
pub use asset::Asset;
pub use ollama_models_info_fetcher::{Model, ModelBuilder};
use rig::providers::ollama::Client;
pub static OM_CLIENT: Lazy<Client> = Lazy::new(Client::new);
