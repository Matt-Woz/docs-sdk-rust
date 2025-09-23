use crate::examples_error::ExamplesError;
use crate::timeout::timeout;
use couchbase::collection::Collection;
use couchbase::durability_level::DurabilityLevel;
use couchbase::options::kv_options::{GetOptions, InsertOptions, ReplaceOptions};
use couchbase::results::kv_results::MutationResult;
use serde_json::json;
use std::time::Duration;

pub async fn upsert(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::upsert[]
    let doc = json!({
            "foo": "bar",
            "baz": "qux",
    });

    match collection.upsert("document-key", doc, None).await {
        Ok(_result) => {
            println!("Document upsert successful");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
    // #end::upsert[]

    Ok(())
}

pub async fn insert(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::insert[]
    let doc = json!({
            "foo": "bar",
            "baz": "qux",
    });

    match collection.insert("document-key", doc, None).await {
        Ok(_result) => {
            println!("Document insert successful");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
    // #end::insert[]

    Ok(())
}

pub async fn remove(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::remove[]
    match collection.remove("document-key", None).await {
        Ok(_result) => {
            println!("Document remove successful");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
    // #end::remove[]

    Ok(())
}

pub async fn get(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::get[]
    let result = collection.get("document-key", None).await?;
    // #end::get[]

    Ok(())
}

pub async fn insert_then_get(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::insert-then-get[]
    let doc = json!({
            "status": "bar",
    });

    collection.insert("document-key", doc, None).await?;

    let result = collection.get("document-key", None).await?;
    let value: serde_json::Value = result.content_as()?;
    // Unwrap for simplicity.
    let status = value.get("status").unwrap();
    let status = status.as_str().unwrap();

    println!("Couchbase is ${status}");
    // #end::insert-then-get[]

    Ok(())
}

// tag::cas-loop[]
pub async fn cas_loop(
    collection: Collection,
    doc_id: String,
    guard: Option<u8>,
) -> Result<MutationResult, ExamplesError> {
    let mut guard = guard.unwrap_or(10);

    loop {
        // Get the current document contents
        let get_result = collection.get(&doc_id, None).await?;

        let mut content: serde_json::Value = get_result.content_as()?;
        let visit_count = content["visitCount"].as_u64().unwrap_or(0);

        content["visitCount"] = json!(visit_count + 1);

        match collection
            .replace(
                &doc_id,
                content,
                ReplaceOptions::new().cas(get_result.cas()),
            )
            .await
        {
            Ok(result) => return Ok(result),
            Err(e) => match e.kind() {
                couchbase::error::ErrorKind::CasMismatch => {
                    if guard > 0 {
                        guard -= 1;
                    } else {
                        return Err(ExamplesError::from(e));
                    }
                }
                _ => {
                    return Err(ExamplesError::from(e));
                }
            },
        };
    }
}
// end::cas-loop[]

pub async fn locking(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::locking[]
    let get_result = collection
        .get_and_lock("key", Duration::from_secs(10), None)
        .await?;

    collection.unlock("key", get_result.cas(), None).await?;
    // #end::locking[]

    Ok(())
}

pub async fn document_not_found(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::replace-document-not-found[]
    let doc = json!({
            "foo": "bar",
            "baz": "qux",
    });

    match collection.replace("does-not-exist", doc, None).await {
        Ok(_result) => {
            println!("Document upsert successful");
        }
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::DocumentNotFound => {
                println!("Document not found");
            }
            _ => println!("Error: {e}"),
        },
    }
    // #end::replace-document-not-found[]

    // #tag::get-document-not-found[]
    match collection.get("does-not-exist", None).await {
        Ok(_result) => {
            println!("Document get successful");
        }
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::DocumentNotFound => {
                println!("Document not found");
            }
            _ => println!("Error: {e}"),
        },
    }
    // #end::get-document-not-found[]

    Ok(())
}

pub async fn document_exists(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::insert-exists[]
    let doc = json!({
            "foo": "bar",
            "baz": "qux",
    });

    match collection.insert("does-already-exist", doc, None).await {
        Ok(_result) => {
            println!("Document upsert successful");
        }
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::DocumentExists => {
                println!("Document exists");
            }
            _ => println!("Error: {e}"),
        },
    }
    // #end::insert-exists[]

    Ok(())
}

pub async fn cas_mismatch(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::cas-mismatch[]
    let mut guard = 3;
    let new_json = json!({"foo": "bar"});

    let doc = collection.get("doc", None).await?;
    match collection
        .replace("doc", &new_json, ReplaceOptions::new().cas(doc.cas()))
        .await
    {
        Ok(_result) => {
            println!("Document replace successful");
            return Ok(());
        }
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::CasMismatch => {
                guard -= 1;
                if guard > 0 {
                    println!("CAS mismatch, retry up to {guard} times");
                } else {
                    return Err(ExamplesError::from(e));
                }
            }
            _ => {
                println!("Error: {e}");
                return Err(ExamplesError::from(e));
            }
        },
    };
    // #end::cas-mismatch[]

    Ok(())
}

pub async fn durability_ambiguous(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::durability-ambiguous[]
    let mut guard = 3;
    let doc = json!({"foo": "bar"});

    loop {
        let result = collection
            .insert(
                "doc-id",
                &doc,
                InsertOptions::new().durability_level(DurabilityLevel::MAJORITY),
            )
            .await;

        match result {
            Ok(_result) => {
                println!("Document insert successful");
                return Ok(());
            }
            Err(e) => match e.kind() {
                couchbase::error::ErrorKind::DurabilityAmbiguous => {
                    // For ambiguous errors on inserts, simply retry them.
                    guard -= 1;
                    if guard > 0 {
                        println!("Durability ambiguous, retry up to {guard} times");
                    } else {
                        return Err(ExamplesError::from(e));
                    }
                }
                couchbase::error::ErrorKind::DocumentExists => {
                    // The logic here is that if we failed to insert on the first attempt then
                    // it's a true error, otherwise we retried due to an ambiguous error, and
                    // it's ok to continue as the operation was actually successful.
                    println!("Document insert successful");
                    return Ok(());
                }
                _ => {
                    println!("Error: {e}");
                    return Err(ExamplesError::from(e));
                }
            },
        }
    }
    // #end::durability-ambiguous[]
}

pub async fn insert_real(collection: Collection) -> Result<(), ExamplesError> {
    // tag::insert-real[]
    let initial_guard = 3;
    let base_delay = Duration::from_millis(10);
    let mut current_backoff_factor = 1;
    let mut guard = initial_guard;
    let doc = json!({
            "foo": "bar",
            "baz": "qux",
    });
    let doc_id = "document-key";

    loop {
        let result = timeout(
            Duration::from_millis(2500),
            collection.insert(
                &doc_id,
                &doc,
                InsertOptions::new().durability_level(DurabilityLevel::MAJORITY),
            ),
        )
        .await;

        match result {
            Ok(_result) => {
                println!("Document insert successful");
                return Ok(());
            }
            Err(e) => match e.kind() {
                couchbase::error::ErrorKind::DocumentExists => {
                    // The logic here is that if we failed to insert on the first attempt then
                    // it's a true error, otherwise we retried due to an ambiguous error, and
                    // it's ok to continue as the operation was actually successful.
                    if guard == initial_guard {
                        return Err(ExamplesError::from(e));
                    }

                    println!("Document insert successful");
                    return Ok(());
                }
                couchbase::error::ErrorKind::DurabilityAmbiguous
                // Temporary/transient errors that are likely to be resolved
                // on a retry
                | couchbase::error::ErrorKind::TemporaryFailure
                | couchbase::error::ErrorKind::DurabilityWriteInProgress
                | couchbase::error::ErrorKind::DurableWriteRecommitInProgress
                // These transient errors won't be returned on an insert, but can be used
                // when writing similar wrappers for other mutation operations
                | couchbase::error::ErrorKind::CasMismatch => {
                    if guard > 0 {
                        tokio::time::sleep(base_delay * current_backoff_factor).await;
                        current_backoff_factor *= 2;
                        guard -= 1;
                        println!("Transient error, retry up to {guard} times");
                    } else {
                        return Err(ExamplesError::from(e));
                    }
                }
                _ => {
                    println!("Error: {e}");
                    return Err(ExamplesError::from(e));
                }
            },
        }
    }
    // end::insert-real[]
}

pub async fn replace_with_cas(collection: Collection) -> Result<(), ExamplesError> {
    // tag::replace-with-cas[]
    let doc_id = "document-key";

    let doc = json!({"status": "great"});

    // Insert a document.  Don't care about the exact details of the result so just propagate
    // the error.
    collection.insert(&doc_id, &doc, None).await?;

    // Get the document back
    let get_result = collection.get(doc_id, None).await?;
    let mut value: serde_json::Value = get_result.content_as()?;
    value["status"] = json!("awesome");

    // Replace the document with the updated content, and the document's CAS value
    // (which we'll cover in a moment)
    match collection
        .replace(doc_id, value, ReplaceOptions::new().cas(get_result.cas()))
        .await
    {
        Ok(_result) => {
            println!("Document replace successful");
        }
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::CasMismatch => {
                println!("Could not write as another agent has concurrently modified the document");
            }
            _ => println!("Error: {e}"),
        },
    }
    // end::replace-with-cas[]

    Ok(())
}

pub async fn replace_retry(collection: Collection) -> Result<(), ExamplesError> {
    // tag::replace-retry[]
    let doc_id = "document-key";

    let doc = json!({"status": "great"});

    collection.insert(&doc_id, &doc, None).await?;

    let op = async || {
        let get_result = collection.get(doc_id, None).await?;
        let mut value: serde_json::Value = get_result.content_as()?;
        value["status"] = json!("awesome");

        collection
            .replace(doc_id, value, ReplaceOptions::new().cas(get_result.cas()))
            .await
    };

    // Send our lambda to retryOnCASMismatch to take care of retrying it
    // For space reasons, error-handling of r is left out
    retry_on_cas_mismatch(op).await?;

    Ok(())
}

async fn retry_on_cas_mismatch<Fut>(
    operation: impl Fn() -> Fut,
) -> Result<MutationResult, ExamplesError>
where
    Fut: Future<Output = Result<MutationResult, couchbase::error::Error>>,
{
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => match e.kind() {
                couchbase::error::ErrorKind::CasMismatch => {
                    continue;
                }
                _ => {
                    return Err(ExamplesError::from(e));
                }
            },
        };
    }
}
// end::replace-retry[]

pub async fn remove_with_durability(collection: Collection) -> Result<(), ExamplesError> {
    // tag::remove-with-durability[]
    match collection
        .remove(
            "document-key",
            couchbase::options::kv_options::RemoveOptions::new()
                .durability_level(DurabilityLevel::MAJORITY),
        )
        .await
    {
        Ok(_result) => {
            println!("Document remove successful");
        }
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::DocumentNotFound => {
                println!("Document not found");
            }
            _ => println!("Error: {e}"),
        },
    }
    // end::remove-with-durability[]

    Ok(())
}

pub async fn insert_with_expiry(collection: Collection) -> Result<(), ExamplesError> {
    // tag::insert-with-expiry[]
    let doc = json!({
            "foo": "bar",
            "baz": "qux",
    });

    collection
        .insert(
            "document-key",
            doc,
            InsertOptions::new().expiry(Duration::from_secs(10)),
        )
        .await?;
    // end::insert-with-expiry[]

    Ok(())
}

pub async fn get_with_expiry(collection: Collection) -> Result<(), ExamplesError> {
    // tag::get-with-expiry[]
    let result = collection
        .get("document-key", GetOptions::new().expiry(true))
        .await?;

    if let Some(expiry) = result.expiry_time() {
        println!("Document expires at {expiry}");
    } else {
        println!("Document does not have an expiry");
    }
    // end::get-with-expiry[]

    Ok(())
}

pub async fn replace_with_preserve_expiry(collection: Collection) -> Result<(), ExamplesError> {
    // tag::replace-with-preserve-expiry[]
    let doc_id = "document-key";

    let doc = json!({"status": "great"});

    collection
        .replace(doc_id, doc, ReplaceOptions::new().preserve_expiry(true))
        .await?;
    // end::replace-with-preserve-expiry[]

    Ok(())
}

pub async fn get_and_touch(collection: Collection) -> Result<(), ExamplesError> {
    // tag::get-and-touch[]
    let result = collection
        .get_and_touch("document-key", Duration::from_secs(60 * 4), None)
        .await?;
    // end::get-and-touch[]

    Ok(())
}
