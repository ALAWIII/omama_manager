use super::{get_omamadb_connection, ODatabse};
use super::{OChat, OMessage};
use crate::Result;
use surrealdb::sql::{Id, Thing};

const FETCH_ALL_MESSAGES: &str = r#"SELECT VALUE ->mess_chat->message as message FROM type::thing("chat", $c_id) FETCH message ;"#;
const FETCH_SUMMARY: &str = r#"SELECT VALUE summary FROM type::thing("chat", $c_id);"#;
const STORE_SUMMARY: &str = r#"UPSERT type::thing("chat",$c_id) SET summary = $summary;"#;
// const FETCH_ALL_CHATS: &str = "SELECT id,name,summary FROM chat";

pub async fn get_all_chats() -> Result<Vec<OChat>> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    let chats = db.select("chat").await?;

    Ok(chats)
}
pub async fn get_chat_by_id(id: i64) -> Result<OChat> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;
    let mut result = db
        .query("SELECT id,name FROM type::thing('chat',$id)")
        .bind(("id", id))
        .await?;
    let res: Option<OChat> = result.take(0)?;
    Ok(res.unwrap_or_default())
}

pub async fn get_all_messages(c_id: i64) -> Result<Vec<OMessage>> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    let mut resp = db.query(FETCH_ALL_MESSAGES).bind(("c_id", c_id)).await?;
    //dbg!(&resp);
    let msgs: Vec<Vec<OMessage>> = resp.take(0)?;
    let msgs = msgs.into_iter().flatten().collect();
    Ok(msgs)
}

//--------------------------------encapsulated by create_chat-------------------------
//------------------------summary---------------------
pub async fn get_summary_of_chat(c_id: i64) -> Result<String> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    let mut resp = db.query(FETCH_SUMMARY).bind(("c_id", c_id)).await?;
    let summary: Vec<String> = resp.take(0)?;
    Ok(summary.first().unwrap_or(&"".to_owned()).to_owned())
}

pub async fn store_summary_of_chat(c_id: i64, summary: &str) -> Result<()> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    db.query(STORE_SUMMARY)
        .bind(("c_id", c_id))
        .bind(("summary", summary.to_owned()))
        .await?;
    Ok(())
}

//----------------------message---
pub async fn insert_message(msg: OMessage) -> Result<OMessage> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;

    let omsg: Option<OMessage> = db.create(("message", *msg.id())).content(msg).await?;
    Ok(omsg.unwrap_or_default())
}
pub async fn relate_m_c(c_id: i64, m_id: i64) -> Result<()> {
    let chat_id = Thing::from(("chat", Id::Number(c_id)));
    let msg_id = Thing::from(("message", Id::Number(m_id)));
    let db = get_omamadb_connection(ODatabse::Ochat).await;
    db.query(format!("RELATE {} ->mess_chat -> {};", chat_id, msg_id))
        .await?;
    Ok(())
}
//-------------------------------------
pub async fn insert_chat(o_chat: OChat) -> Result<OChat> {
    let db = get_omamadb_connection(ODatabse::Ochat).await;
    let c: Option<OChat> = db.create(("chat", *o_chat.id())).content(o_chat).await?;
    Ok(c.unwrap_or_default())
}
