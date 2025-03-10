use crate::OResult;
use ollama_rs::{
    models::{pull::PullModelStatusStream, LocalModel},
    Ollama,
};
use once_cell::sync::Lazy;
use std::future::Future;
static CLIENT: Lazy<Ollama> = Lazy::new(Ollama::default);

pub async fn download_model(name: &str, token_size: &str) -> OResult<()> {
    let model_name = format!("{}:{}", name, token_size);
    CLIENT.pull_model(model_name, false).await?;
    Ok(())
}

pub async fn download_model_stream<F, Fut>(name: &str, token_size: &str, f_stream: F) -> OResult<()>
where
    F: FnOnce(PullModelStatusStream) -> Fut,
    Fut: Future,
{
    let model_name = format!("{}:{}", name, token_size);
    let stream = CLIENT.pull_model_stream(model_name, false).await?;
    f_stream(stream).await;
    Ok(())
}
pub async fn delete_model(name: &str, token_size: &str) -> OResult<()> {
    let model_name = format!("{}:{}", name, token_size);
    CLIENT.delete_model(model_name).await?;
    Ok(())
}

pub async fn list_downloaded_models() -> OResult<Vec<LocalModel>> {
    Ok(CLIENT.list_local_models().await?)
}
