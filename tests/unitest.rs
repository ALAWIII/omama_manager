use ollama_rs::models::pull::PullModelStatusStream;
use omama_manager::Result;
use omama_manager::database::{
    self, OChat, ODatabse, OMessage, get_all_chats, get_all_messages, get_omamadb_connection,
    get_summary_of_chat, insert_message, relate_m_c, store_summary_of_chat,
};
use omama_manager::rag::{
    Document, EMBEDDING_MODEL, generate_embeddings,
    generation::generate_response,
    query::{extract_queries, rewrite_query},
    search_similar_docs, store_document,
};
use omama_manager::{
    Model, ModelBuilder,
    chat::{OConfig, create_message},
    service_utils::*,
};
use rig::completion::CompletionError;
use rig::embeddings::EmbeddingModel;
use rig::providers::azure::O1;
use rig::streaming::StreamingResult;
use std::ops::Deref;
use std::path::Path;
use tokio::io::{AsyncWriteExt, stdout};
use tokio_stream::StreamExt;

//------------------------------------
#[tokio::test]
async fn check_web_models_info() -> Result<()> {
    let current_path = get_current_path().unwrap().join("models.json");
    let res = load_models_from_web_to_json().await?;

    assert_eq!(res, current_path.to_string_lossy().to_string());
    Ok(())
}

#[tokio::test]
async fn check_local_models_info() -> Result<()> {
    load_models_from_web_to_json().await?;
    let loaded_models = load_models_from_json_file().await;
    //dbg!(loaded_models);
    assert!(loaded_models.is_ok());
    Ok(())
}

//----------------------------------------------
#[tokio::test]
async fn setup_service() -> Result<()> {
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
async fn listing_local_models() -> Result<()> {
    let models = get_local_models_info().await?;
    dbg!(&models);
    assert!(!models.iter().any(|m| m.name().contains("nomic-embed-text")));
    Ok(())
}
//--------
#[tokio::test]
async fn load_m_web_db() {
    let state = fetch_models_from_web_to_db().await;
    assert!(state.is_ok());
}
//-------
async fn stream_download_helper(mut status: PullModelStatusStream) -> Result<()> {
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
async fn stream_m_download() {
    let m_download = download_model_stream("qwen2.5", "0.5b", stream_download_helper).await;
    dbg!(&m_download);
    assert!(m_download.is_ok());
}

//----------------------------------database tests------------------

#[tokio::test]
async fn check_ochat_connection() {
    let path =
        Path::new("/run/media/allawiii/projects/zed/Rust/omama_manager/target/debug/deps/omamadb");

    database::get_omamadb_connection(ODatabse::Ochat).await;
    assert!(path.exists());
}

#[tokio::test]
async fn fetch_all_chat() -> Result<()> {
    let chats = get_all_chats().await?;
    dbg!(&chats);
    assert!(!chats.is_empty());
    Ok(())
}

#[tokio::test]
async fn fetch_all_messages() -> Result<()> {
    let messages = get_all_messages(1743076482649).await?;

    //dbg!(&messages);
    assert_eq!(messages[0].message(), "how are you?");
    assert_eq!(messages[0].response(), "am fine");
    assert_eq!(*messages[0].id(), 1743076482);
    Ok(())
}

#[tokio::test]
async fn fetch_summary_of_chat() -> Result<()> {
    let summary = get_summary_of_chat(1743076482649).await?;
    assert!(!summary.is_empty());
    assert_eq!(summary, "new world");
    Ok(())
}
#[tokio::test]
async fn check_odoc_connection() {
    let db = get_omamadb_connection(ODatabse::Odoc).await;
}
//--------------------------rag tests--------------------

#[tokio::test]
async fn fetch_embeddings() -> Result<()> {
    let embds = generate_embeddings("shawarma").await?;

    //dbg!(&embds);
    assert!(!embds.embedding.is_empty());
    Ok(())
}
#[tokio::test]
async fn compare_length_embeddings() -> Result<()> {
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
async fn store_docs() -> Result<()> {
    let expected = generate_embeddings("allawiii").await?;
    let doc = store_document(expected.clone()).await?;
    dbg!(&doc);
    assert_eq!(doc, expected);
    Ok(())
}

async fn generate_docs() -> Vec<Document> {
    let words = [
        "hello,world",
        "eid saeed",
        "eid mubark",
        "shawarma is too healthy",
        "chicken are so cute",
    ];
    let mut words_doc = vec![];
    for w in words {
        words_doc.push(generate_embeddings(w).await.unwrap());
    }
    words_doc
}
#[tokio::test]
async fn store_n_docs() {
    let docs = generate_docs().await;
    for d in docs {
        store_document(d).await;
    }
}

#[tokio::test]
pub async fn check_similar_search() -> Result<()> {
    let search_doc = generate_embeddings("chicken").await?;
    let docs = search_similar_docs(search_doc, 1, 0.0).await?;
    dbg!(&docs);
    assert!(!docs.is_empty());
    Ok(())
}
#[tokio::test]
async fn decompose_queries_test() {
    let rewrite_q = rewrite_query(
        "qwen2.5:1.5b",
        "Tell me about the history of wars and its impact on civilization.",
    )
    .await
    .unwrap();
    dbg!(&rewrite_q);
    let queries = extract_queries("qwen2.5:1.5b", &rewrite_q).await;
    dbg!(queries);
}
//----------------test generation------------
#[tokio::test]
async fn generate_response_test() {
    let prompt = "Tell me about the history of wars in short statement.";
    let mut response = generate_response("qwen2.5:1.5b", prompt).await.unwrap();
    while let Some(Ok(w)) = response.next().await {
        print!("{w}");
    }
}
//----------------------------------------test chat-----------------

async fn stream_chat(mut streamer: StreamingResult) -> String {
    let mut response = "".to_owned();
    while let Some(Ok(word)) = streamer.next().await {
        response.push_str(&word.to_string());
        print!("{}", word);
    }
    response
}
#[tokio::test]
async fn check_create_message() {
    let conf = OConfig {
        user_message: "Tell me about the history of wars in short statement".to_string(),
        c_id: 1743076482649,
        model: ModelBuilder::new().name("deepseek-r1:1.5b").build(),
    };
    let resp = create_message(conf, stream_chat).await;
    //assert!(resp.is_ok());
}
#[tokio::test]
async fn check_summary_store() {
    let summary = "lets eat shawarma";
    let st = store_summary_of_chat(1743076482649, summary).await;
    assert!(st.is_ok());
}
//----------------//-----////--

#[tokio::test]
async fn check_message_insertion() -> Result<()> {
    let omsg = OMessage::new();
    let id = *omsg.id();
    let msg_inserted = insert_message(omsg).await?;
    assert_eq!(id, *msg_inserted.id());
    Ok(())
}

#[tokio::test]
async fn check_relate_m_c() -> Result<()> {
    let omsg = OMessage::new();
    let id = *omsg.id();
    let msg_inserted = insert_message(omsg).await?;
    relate_m_c(1743076482649, id).await;
    Ok(())
}

//-----------------------test create_chat-------------
#[tokio::test]
async fn check_message_creation() -> Result<()> {
    let conf = OConfig {
        user_message: "Tell me about the history of wars in short statement".to_string(),
        c_id: 1743678992662,
        model: ModelBuilder::new().name("deepseek-r1:1.5b").build(),
    };
    let msg = create_message(conf, stream_chat).await;
    Ok(())
}
//---
#[tokio::test]
async fn updating_chat_name() {
    let mut o = OChat::new();
    let mut o = OChat {
        id: 1743678992662,
        ..o
    };
    o.update_name("new era of ducking").await;
}
