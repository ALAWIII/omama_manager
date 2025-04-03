use super::super::get_current_path;
use crate::database::{get_omamadb_connection, ODatabse};
use crate::OResult;
use ollama_models_info_fetcher::{
    convert_to_json, fetch_all_available_models, fetch_model_info, Model,
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::io::{BufReader, Write};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OModelInfo {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: String,
    #[serde(flatten)]
    pub model: Model,
}

fn deserialize_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let thing = Thing::deserialize(deserializer)?;
    if let surrealdb::sql::Id::String(num) = thing.id {
        Ok(num)
    } else {
        Err(serde::de::Error::custom("Expected a numeric ID"))
    }
}

/// get all models to json , returns the path as string!!!
pub async fn load_models_from_web_to_json() -> OResult<String> {
    let json_path = get_current_path()?.join("models.json");

    let mut f = File::options()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(&json_path)?;

    let models = fetch_all_available_models().await?;

    let mut models_info = vec![];

    for model_name in models {
        models_info.push(fetch_model_info(&model_name).await?);
    }
    let to_json = convert_to_json(&models_info)?;

    f.write_all(to_json.as_bytes())?;

    Ok(json_path.to_string_lossy().to_string())
}

pub async fn load_models_from_json_file() -> OResult<Vec<Model>> {
    let file_json = get_current_path()?.join("models.json");
    let file = File::open(&file_json)?;
    let reader = BufReader::new(file);
    let models: Vec<Model> = from_reader(reader)?;
    Ok(models)
}

pub async fn fetch_models_from_web_to_db() -> OResult<()> {
    let models = fetch_all_available_models().await?;
    let db = get_omamadb_connection(ODatabse::Ochat).await;
    for model_name in models {
        if let Ok(m) = fetch_model_info(&model_name).await {
            let mi = OModelInfo {
                id: m.name().to_string(),
                model: m,
            };
            db.upsert::<Option<OModelInfo>>(("model", &mi.id))
                .content(mi)
                .await?;
        }
    }

    Ok(())
}
pub async fn fetch_models_from_db() -> OResult<Vec<OModelInfo>> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;
    let resp: Vec<OModelInfo> = db.select("model").await?;
    Ok(resp)
}

#[cfg(test)]
mod quick_test {
    use super::OModelInfo;
    use ollama_models_info_fetcher::{fetch_all_available_models, fetch_model_info};
    use ollama_td::OResult;

    use crate::{
        database::{get_omamadb_connection, ODatabse},
        service_utils::{fetch_models_from_db, fetch_models_from_web_to_db},
    };

    #[tokio::test]
    async fn check_models_from_web_to_db() -> OResult<()> {
        let models = fetch_all_available_models().await?;
        let db = get_omamadb_connection(ODatabse::Ochat).await;
        if let Ok(m) = fetch_model_info(&models[0]).await {
            let mi = OModelInfo {
                id: m.name().to_string(),
                model: m,
            };
            let v: Option<OModelInfo> = db.upsert(("model", &mi.id)).content(mi).await?;
            //dbg!(&v);
            assert!(v.is_some());
        }
        Ok(())
    }
    #[tokio::test]
    async fn check_models_from_db() -> OResult<()> {
        fetch_models_from_web_to_db().await?;
        let models = fetch_models_from_db().await?;
        //dbg!(&models);
        assert!(!models.is_empty());
        Ok(())
    }
}
