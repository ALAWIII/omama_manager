use super::{get_omamadb_connection, ODatabse};
use super::{OChat, OMessage};
use crate::OResult;

const FETCH_ALL_MESSAGES: &str = r#"SELECT VALUE ->mess_chat->message as message FROM type::thing("chat", $c_id) FETCH message ;"#;
const FETCH_SUMMARY: &str = r#"SELECT VALUE summary FROM type::thing("chat", $c_id);"#;
const STORE_SUMMARY: &str = r#"UPSERT type::thing("chat",$c_id) SET summary = $summary;"#;
const CREATE_MESSAGE: &str = r#""#;
// const FETCH_ALL_CHATS: &str = "SELECT id,name,summary FROM chat";

pub async fn get_all_chats() -> OResult<Vec<OChat>> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    let chats = db.select("chat").await?;

    Ok(chats)
}

pub async fn get_all_messages(c_id: i64) -> OResult<Vec<OMessage>> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    let mut resp = db.query(FETCH_ALL_MESSAGES).bind(("c_id", c_id)).await?;
    //dbg!(&resp);
    let msgs: Vec<Vec<OMessage>> = resp.take(0)?;
    let msgs = msgs.into_iter().flatten().collect();
    Ok(msgs)
}

pub async fn get_summary_of_chat(c_id: i64) -> OResult<String> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    let mut resp = db.query(FETCH_SUMMARY).bind(("c_id", c_id)).await?;
    let mut summary: Vec<String> = resp.take(0)?;
    Ok(summary.remove(0))
}

pub async fn store_summary_of_chat(c_id: i64, summary: &str) -> OResult<()> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    db.query(STORE_SUMMARY)
        .bind(("c_id", c_id))
        .bind(("summary", summary.to_owned()))
        .await?;
    Ok(())
}

pub async fn insert_chat() {}
pub async fn insert_message(msg: OMessage) -> OResult<OMessage> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    let omsg: Option<OMessage> = db.create(("message", *msg.id())).content(msg).await?;
    Ok(omsg.unwrap_or_default())
}
