use crate::examples_error::ExamplesError;
use couchbase::collection::Collection;
use couchbase::options::kv_binary_options::IncrementOptions;

pub async fn counter_example(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::counterexample[]
    let counter_doc_id = "counter-doc";
    // Increment by 1, creating doc if needed
    collection.binary().increment(&counter_doc_id, None).await?;
    // Decrement by 1
    collection.binary().decrement(&counter_doc_id, None).await?;
    // Increment by 5
    collection
        .binary()
        .increment(&counter_doc_id, IncrementOptions::new().delta(5))
        .await?;
    // #end::counterexample[]

    Ok(())
}
