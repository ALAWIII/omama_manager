use crate::Result;
use crate::{
    database::{
        get_summary_of_chat, insert_chat, insert_message, relate_m_c, store_summary_of_chat, OChat,
        OMessage,
    },
    rag::{generate_embeddings, search_similar_docs, store_document, Document},
    Model, OM_CLIENT,
};
pub use rig::streaming::StreamingResult;
use rig::{
    completion::{CompletionError, Prompt},
    streaming::StreamingPrompt,
};

#[derive(Debug, Default)]
pub struct OConfig {
    pub user_message: String,
    pub c_id: i64,
    pub model: Model,
}

/// accepts a message and a chat id (to store and retrieve a summary).
pub async fn create_message(
    config: OConfig,
    f_stream: impl AsyncFn(StreamingResult) -> String,
) -> Result<(), CompletionError> {
    let msg_doc = generate_embeddings(&config.user_message).await.unwrap();
    let docs = merge_docs(msg_doc).await;
    let context = get_summary_of_chat(config.c_id)
        .await
        .unwrap_or("".to_string());
    let agent = OM_CLIENT
        .agent(config.model.name())
        .preamble("you are a perfect AI assistant in all disciplines.")
        .context(&context)
        .context(&format!("information:\n{docs}"))
        .build();
    let resp = f_stream(agent.stream_prompt(&config.user_message).await?).await;
    //----------------------storing the message in ochat db---------------------
    let mut oms = OMessage::new();
    oms.add_message(&config.user_message);
    oms.add_response(&resp);
    let oms = insert_message(oms).await.unwrap_or_default();
    relate_m_c(config.c_id, *oms.id()).await.unwrap_or_default();
    //---------------------------------generating summary and store them in ochat + odoc db's--------------------
    let prompt = format!("user:{}\nAI:{}", config.user_message, resp);
    let summary = create_summary(config.model.name(), prompt).await;
    dbg!(&summary);
    store_summary_of_chat(config.c_id, &summary)
        .await
        .unwrap_or_default();
    store_summary_as_doc(summary).await.unwrap();

    Ok(())
}

async fn store_summary_as_doc(summary: String) -> Result<()> {
    let doc = generate_embeddings(&summary).await?;
    store_document(doc).await?;
    Ok(())
}
async fn create_summary(model: &str, prompt: String) -> String {
    let agent_summary = OM_CLIENT
        .agent(model)
        .preamble("You are an AI assistant designed to summarize conversations efficiently. Your task is to extract key points from the discussion while maintaining clarity and coherence. Focus on the most important details, omitting unnecessary repetitions. Format the summary logically, preserving the intent of the conversation. Ensure brevity while maintaining accuracy. If relevant, highlight any conclusions, decisions, or action points discussed.")
        .build();
    agent_summary.prompt(prompt).await.unwrap_or("".to_owned())
}
async fn merge_docs(msg_doc: Document) -> String {
    search_similar_docs(msg_doc, 3, 0.5)
        .await
        .unwrap_or(vec![])
        .into_iter()
        .map(|d| d.content)
        .reduce(|d1, d2| format!("{d1}\n{d2}"))
        .unwrap_or("".into())
}

pub async fn create_chat() -> OChat {
    let o_chat = OChat::new();
    insert_chat(o_chat).await.unwrap_or_default()
}
