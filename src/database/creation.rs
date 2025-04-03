use crate::{Asset, OResult};
use std::fmt::Display;
use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};

use crate::service_utils::get_current_path;
use tokio::sync::OnceCell;

static O_MAMA_DB: OnceCell<Surreal<Db>> = OnceCell::const_new();

pub enum ODatabse {
    Ochat,
    Odoc,
}
impl Display for ODatabse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ochat => "ochat",
                _ => "odoc",
            }
            .to_owned()
        )
    }
}

async fn connect_db() -> OResult<Surreal<Db>> {
    let db_path = get_current_path()?.join("omamadb");
    let db = Surreal::new::<RocksDb>(db_path).await?;
    db.use_ns("o_base").await?;
    let db = create_mamadb(db, "ochat", "ochat_schema.surql").await;
    let db = create_mamadb(db, "odoc", "odoc_schema.surql").await;

    Ok(db)
}
async fn create_mamadb(db: Surreal<Db>, db_name: &str, sql_file: &str) -> Surreal<Db> {
    let o_schema = Asset::get(sql_file).unwrap();
    let schema_query = String::from_utf8(o_schema.data.to_vec());
    db.use_db(db_name)
        .await
        .expect(&format!("Failed to use {db_name}!"));
    db.query(schema_query.unwrap())
        .await
        .expect(&format!("Failed to execute {db_name} schema!"));
    db
}

pub async fn get_omamadb_connection(db_name: ODatabse) -> Surreal<Db> {
    let db = O_MAMA_DB
        .get_or_init(async || {
            connect_db()
                .await
                .expect("Failed to init database connection!")
        })
        .await;
    db.use_db(db_name.to_string())
        .await
        .expect("failed to use the database name!");
    db.clone()
}

// ----------- needs more revision and development-----------------

//------------private tests------------
#[cfg(test)]
mod quick_test {
    use ollama_td::OResult;

    use crate::Asset;

    use super::connect_db;

    async fn execute_queries() -> OResult<()> {
        let o_chat_schema = Asset::get("ochat_schema.surql").unwrap();
        let mut buffer = String::from_utf8(o_chat_schema.data.to_vec());
        let db = connect_db().await?;
        db.query(buffer.unwrap()).await?;
        Ok(())
    }
    #[test]
    fn get_schema_asset_file() {
        let o_chat_schema = Asset::get("ochat_schema.surql").unwrap();
        let mut buffer = String::from_utf8(o_chat_schema.data.to_vec());
        assert!(buffer.is_ok());
        dbg!(buffer.unwrap());
    }
}
