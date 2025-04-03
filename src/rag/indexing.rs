use std::hash::Hash;

use crate::{
    database::{get_omamadb_connection, ODatabse},
    OResult, OM_CLIENT,
};
use nanoid::nanoid;
use once_cell::sync::Lazy;
use rig::{
    embeddings::EmbeddingModel as Em,
    providers::ollama::{EmbeddingModel, NOMIC_EMBED_TEXT},
};
use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::{Id, Thing};

pub static EMBEDDING_MODEL: Lazy<EmbeddingModel> =
    Lazy::new(|| OM_CLIENT.embedding_model(NOMIC_EMBED_TEXT));

fn deserialize_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let thing = Thing::deserialize(deserializer)?;
    match thing.id {
        Id::String(id) => Ok(id),
        _ => Err(serde::de::Error::custom("Expected a String ID")),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: String,
    pub content: String,
    #[serde(skip_deserializing)]
    pub embedding: Vec<f64>,
    #[serde(skip_serializing)]
    pub accuracy: f64,
}
impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Hash for Document {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
pub async fn generate_embeddings(text: &str) -> OResult<Document> {
    let id = nanoid!(12);
    let embedding = (*EMBEDDING_MODEL).embed_text(text).await?;
    Ok(Document {
        id,
        content: embedding.document,
        embedding: embedding.vec,
        accuracy: 0.0,
    })
}

pub async fn store_document(doc: Document) -> OResult<Document> {
    let db = get_omamadb_connection(ODatabse::Odoc).await;
    let docy: Option<Document> = db.create("document").content(doc).await?.unwrap();
    Ok(docy.unwrap())
}
