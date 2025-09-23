use crate::examples_error::ExamplesError;
use couchbase::collection::Collection;
use couchbase::options::kv_options::ReplaceOptions;

pub async fn counter_with_cas(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::counterwithcas[]
    let increment_amount = 5;
    let doc = collection.get("counter-id", None).await?;
    let value = doc.content_as::<u64>()?;
    let cas = doc.cas();

    if should_increment_value(value) {
        collection
            .replace(
                "counter-id",
                value + increment_amount,
                ReplaceOptions::new().cas(cas),
            )
            .await?;
    }
    // #end::counterwithcas[]

    Ok(())
}

fn should_increment_value(value: u64) -> bool {
    true
}
