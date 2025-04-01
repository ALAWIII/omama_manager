use once_cell::sync::Lazy;
pub mod service_utils;
pub use ollama_td::OResult;

mod asset;
mod chat;
pub mod database;
pub mod rag;
pub use asset::Asset;

use rig::providers::ollama::Client;

pub static OM_CLIENT: Lazy<Client> = Lazy::new(Client::new);
