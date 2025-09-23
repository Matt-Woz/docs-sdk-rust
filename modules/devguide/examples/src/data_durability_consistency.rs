use couchbase::collection::Collection;
use couchbase::durability_level::DurabilityLevel;
use couchbase::options::kv_options::RemoveOptions;

pub async fn remove_with_durability(collection: Collection) {
    // #tag::remove_with_durability[]
    match collection
        .remove(
            "document-key",
            RemoveOptions::new().durability_level(DurabilityLevel::MAJORITY),
        )
        .await
    {
        Ok(result) => {
            println!("Document removed successfully: {:?}", result);
        }
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::DocumentNotFound => {
                println!("Document not found");
            }
            _ => {
                println!("Failed to remove document: {}", e);
            }
        },
    }
    // #end::remove_with_durability[]
}
