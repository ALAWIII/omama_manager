use crate::OResult;

use crate::database::{get_omamadb_connection, ODatabse};

use super::Document;

static SEARCH_QUERY: &str = "SELECT id, content ,vector::similarity::cosine(embedding,$emb) as accuracy FROM document WHERE accuracy>=$thresh  ORDER BY accuracy DESC LIMIT $n_doc;";

pub async fn search_similar_docs(
    doc: Document,
    n_doc: usize,
    thresh: f64,
) -> OResult<Vec<Document>> {
    let db = get_omamadb_connection(ODatabse::Odoc).await;
    let docs: Vec<Document> = {
        db.query(SEARCH_QUERY)
            .bind(("n_doc", n_doc))
            .bind(("emb", doc.embedding))
            .bind(("thresh", thresh))
            .await?
            .take(0)?
    };
    Ok(docs)
}
