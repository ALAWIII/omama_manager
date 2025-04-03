use crate::OM_CLIENT;
use rig::{completion::Prompt, extractor::ExtractionError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const RE_PROMPT: &str = "your job is to rewrite the text in 3 different short meaningful forms.";

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct Queries {
    pub queries: Vec<String>,
}
pub async fn rewrite_query(model: &str, query: &str) -> Option<String> {
    let agent = OM_CLIENT.agent(model).preamble(RE_PROMPT).build();
    agent.prompt(query).await.ok()
}

/// must use models that are marked with tools property
pub async fn extract_queries(model: &str, queries: &str) -> Result<Queries, ExtractionError> {
    let agent = OM_CLIENT
        .extractor::<Queries>(model)
        .preamble("extract queries from the given text")
        .build();
    agent.extract(queries).await
}
