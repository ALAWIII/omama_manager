use crate::{
    database::{get_omamadb_connection, ODatabse},
    Model, OResult,
};
use ollama_models_info_fetcher::ModelBuilder;
use ollama_rs::{models::pull::PullModelStatusStream, Ollama};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::future::Future;

const MODEL_INFO: &str = "SELECT * OMIT id FROM type::thing('model',$m_name);";

static OL_CLIENT: Lazy<Ollama> = Lazy::new(Ollama::default);

pub async fn download_model(name: &str, token_size: &str) -> OResult<()> {
    let model_name = format!("{}:{}", name, token_size);
    OL_CLIENT.pull_model(model_name, false).await?;
    Ok(())
}

pub async fn download_model_stream<F, Fut>(name: &str, token_size: &str, f_stream: F) -> OResult<()>
where
    F: FnOnce(PullModelStatusStream) -> Fut,
    Fut: Future,
{
    let model_name = format!("{}:{}", name, token_size);
    let stream = OL_CLIENT.pull_model_stream(model_name, false).await?;
    f_stream(stream).await;
    Ok(())
}
pub async fn delete_model(name: &str, token_size: &str) -> OResult<()> {
    let model_name = format!("{}:{}", name, token_size);
    OL_CLIENT.delete_model(model_name).await?;
    Ok(())
}

pub async fn get_local_models_info() -> OResult<Vec<Model>> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;
    let model = OL_CLIENT
        .list_local_models()
        .await?
        .into_iter()
        .map(|m| {
            let mm = m.name.split_once(":").unwrap();
            (mm.0.to_owned(), mm.1.to_owned())
        })
        .collect::<Vec<_>>();

    let mut table = HashMap::<String, Vec<String>>::new();
    for (key, value) in model {
        table.entry(key).or_default().push(value);
    }
    table.remove("nomic-embed-text");
    let mut local_models_info = vec![];

    for (m_name, token_sizes) in table {
        //let n_varients = Array::from(list);
        let mut response = db
            .query(MODEL_INFO)
            .bind(("m_name", m_name.to_owned()))
            .await?;
        let m_info: Option<Model> = response.take(0)?;
        let m_info = m_info.unwrap_or_default();
        let m_builder = ModelBuilder::new()
            .name(m_info.name())
            .category(m_info.category().clone())
            .summary_content(m_info.summary_content())
            .readme_content(m_info.readme_content())
            .varients(
                m_info
                    .varients()
                    .clone()
                    .into_iter()
                    .filter(|v| token_sizes.contains(&v.token_size().to_string()))
                    .collect(),
            )
            .build();
        local_models_info.push(m_builder);
    }

    Ok(local_models_info)
}
