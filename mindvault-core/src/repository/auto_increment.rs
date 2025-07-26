use bson::{doc, Document};
use mongodb::{Collection, options::FindOneAndUpdateOptions};
use mongodb::options::ReturnDocument;

pub const AUTO_INCREMENT_COLLECTION_NAME: &str = "ref_auto_increment";

pub async fn get_next_id_for_collection(
    counters: &Collection<Document>,
    collection_name: &str,
) -> mongodb::error::Result<i64> {
    let filter = doc! { "_id": collection_name };
    let seq: i64 = 1;
    let update = doc! { "$inc": { "seq": seq } };
    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .upsert(true)
        .build();

    let result = counters
        .find_one_and_update(filter, update)
        .with_options(options)
        .await?
        .expect("Counter document should exist or be created");
    Ok(result.get_i64("seq").unwrap())
}
