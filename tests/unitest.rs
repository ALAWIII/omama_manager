use ollama_rs::models::pull::PullModelStatusStream;
use omama_manager::{
    delete_model, download_model, download_model_stream, get_current_path, install_tool,
    is_installed_globally, is_installed_locally, is_ollama_running, list_downloaded_models,
    load_models_from_json_file, load_models_from_web_to_json, start_ollama_service, OResult,
};
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;

//------------------------------------
#[tokio::test]
async fn check_web_models_info() -> OResult<()> {
    let current_path = get_current_path().unwrap().join("models.json");
    let res = load_models_from_web_to_json().await?;

    assert_eq!(res, current_path.to_string_lossy().to_string());
    Ok(())
}

#[tokio::test]
async fn check_local_models_info() -> OResult<()> {
    load_models_from_web_to_json().await?;
    let loaded_models = load_models_from_json_file().await;
    //dbg!(loaded_models);
    assert!(loaded_models.is_ok());
    Ok(())
}

//----------------------------------------------
#[tokio::test]
async fn setup_service() -> OResult<()> {
    let running = is_ollama_running().await;
    if !running {
        if !is_installed_globally() {
            if !is_installed_locally()? {
                #[cfg(target_os = "linux")]
                install_tool("0788").await?;
                #[cfg(any(target_os = "windows", target_os = "macos"))]
                install_tool(None).await?;
                start_ollama_service(false).await?;
            } else {
                start_ollama_service(false).await?;
            }
        } else {
            start_ollama_service(true).await?;
        }
    }

    Ok(())
}
//---------------------------------models_mangement--------------

#[tokio::test]
async fn model_download() -> OResult<()> {
    download_model("tinyllama", "latest").await?;
    let models = list_downloaded_models().await?;
    let exist = models.iter().any(|e| e.name.contains("tinyllama:latest"));
    assert!(exist);
    Ok(())
}
//--------

async fn stream_download_helper(mut status: PullModelStatusStream) -> OResult<()> {
    while let Some(s) = status.next().await {
        let ms = s?;
        println!(
            "\r{:.2}",
            (ms.completed.unwrap_or(0) as f64 / ms.total.unwrap_or(1) as f64) * 100.0
        );
        stdout().flush().await?;
    }
    Ok(())
}

#[tokio::test]
async fn model_download_stream() -> OResult<()> {
    download_model_stream("tinyllama", "latest", stream_download_helper).await?;
    let models = list_downloaded_models().await?;
    let exist = models.iter().any(|e| e.name.contains("tinyllama:latest"));
    assert!(exist);
    Ok(())
}
