use std::sync::Arc;

use crate::{Asset, OResult};
use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};

use crate::service_utils::get_current_path;
use tokio::sync::OnceCell;

static O_CHAT_DB: OnceCell<Arc<Surreal<Db>>> = OnceCell::const_new();
static O_DOC_DB: OnceCell<Arc<Surreal<Db>>> = OnceCell::const_new();

async fn connect_db(db_name: &str) -> OResult<Arc<Surreal<Db>>> {
    let db_path = get_current_path()?.join("omamadb");
    let db = Surreal::new::<RocksDb>(db_path).await?;

    db.use_ns("o_base").use_db(db_name).await?;
    Ok(Arc::new(db))
}

pub async fn get_ochatdb_connection() -> Arc<Surreal<Db>> {
    O_CHAT_DB
        .get_or_init(async || {
            let o_schema = Asset::get("ochat_schema.surql").unwrap();
            let schema_query = String::from_utf8(o_schema.data.to_vec());
            let db = connect_db("ochat").await.unwrap();
            db.query(schema_query.unwrap())
                .await
                .expect("Failed to execute ochat schema queries!");
            db
        })
        .await
        .clone()
}

// ----------- needs more revision and development-----------------
pub async fn get_odocdb_connection() -> Arc<Surreal<Db>> {
    O_DOC_DB
        .get_or_init(async || {
            let o_schema = Asset::get("odoc_schema.surql").unwrap();
            let schema_query = String::from_utf8(o_schema.data.to_vec());
            let db = connect_db("odoc").await.unwrap();
            db.query(schema_query.unwrap())
                .await
                .expect("Failed to execute odoc schema queries!");
            db
        })
        .await
        .clone()
}

//------------private tests------------
#[cfg(test)]
mod quick_test {
    use ollama_td::OResult;

    use crate::Asset;

    use super::connect_db;

    async fn execute_queries() -> OResult<()> {
        let o_chat_schema = Asset::get("ochat_schema.surql").unwrap();
        let mut buffer = String::from_utf8(o_chat_schema.data.to_vec());
        let db = connect_db("ochat").await?;
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
