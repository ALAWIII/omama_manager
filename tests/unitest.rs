use ollama_rs::models::pull::PullModelStatusStream;
use omama_manager::database::{
    self, get_all_chats, get_all_messages, get_odocdb_connection, get_summary_of_chat,
};
use omama_manager::rag::{generate_embeddings, store_document, EMBEDDING_MODEL};
use omama_manager::service_utils::*;
use omama_manager::OResult;
use rig::embeddings::EmbeddingModel;
use std::path::Path;
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
//----------------------------------database tests------------------

#[tokio::test]
async fn check_ochat_connection() {
    let path =
        Path::new("/run/media/allawiii/projects/zed/Rust/omama_manager/target/debug/deps/omamadb");

    database::get_ochatdb_connection().await;
    assert!(path.exists());
}

#[tokio::test]
async fn fetch_all_chat() -> OResult<()> {
    let chats = get_all_chats().await?;
    dbg!(&chats);
    assert!(!chats.is_empty());
    Ok(())
}

#[tokio::test]
async fn fetch_all_messages() -> OResult<()> {
    let messages = get_all_messages(1743076482649).await?;

    //dbg!(&messages);
    assert_eq!(messages[0].message(), "how are you?");
    assert_eq!(messages[0].response(), "am fine");
    assert_eq!(*messages[0].id(), 1743076482);
    Ok(())
}

#[tokio::test]
async fn fetch_summary_of_chat() -> OResult<()> {
    let summary = get_summary_of_chat(1743076482649).await?;
    assert!(!summary.is_empty());
    assert_eq!(summary, "new world");
    Ok(())
}
#[tokio::test]
async fn check_odoc_connection() {
    let db = get_odocdb_connection().await;
}
//--------------------------rag tests--------------------

#[tokio::test]
async fn fetch_embeddings() -> OResult<()> {
    let embds = generate_embeddings("shawarma").await?;

    //dbg!(&embds);
    assert!(!embds.embedding.is_empty());
    Ok(())
}
#[tokio::test]
async fn compare_length_embeddings() -> OResult<()> {
    let free = generate_embeddings("free").await?;
    let pals = generate_embeddings("palastine and hamas are heros").await?;
    //dbg!(&embds);
    dbg!(&free.embedding.len());
    dbg!(&pals.embedding.len());
    assert_eq!(free.embedding.len(), pals.embedding.len());
    Ok(())
}
#[tokio::test]
async fn test_embedding_model() {
    let model = EMBEDDING_MODEL.embed_text("text").await.unwrap();
}
#[tokio::test]
async fn store_docs() -> OResult<()> {
    let expected = generate_embeddings("allawiii").await?;
    let doc = store_document(expected.clone()).await?;
    dbg!(&doc);
    assert_eq!(doc, expected);
    Ok(())
}
