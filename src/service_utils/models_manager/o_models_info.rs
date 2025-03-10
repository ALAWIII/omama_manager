use super::super::get_current_path;
use crate::OResult;
use ollama_models_info_fetcher::{
    convert_to_json, fetch_all_available_models, fetch_model_info, Model,
};
use serde_json::from_reader;
use std::fs::File;
use std::io::{BufReader, Write};

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
