use std::time::Duration;
use couchbase::authenticator::{Authenticator, PasswordAuthenticator};
use couchbase::cluster::Cluster;
use crate::examples_error::ExamplesError;
use couchbase::collection::Collection;
use couchbase::options::cluster_options::ClusterOptions;
use couchbase::options::kv_options::ReplaceOptions;
use futures_util::future::join_all;
use futures_util::stream::FuturesUnordered;
use serde_json::json;
use futures::StreamExt;

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

// tag::tokio-main[]
#[tokio::main]
async fn main() {
    let cluster = Cluster::connect(
        "couchbase://localhost",
        ClusterOptions::new(Authenticator::PasswordAuthenticator(
            PasswordAuthenticator::new("username", "password"),
        )),
    )
        .await
        .unwrap();

    let bucket = cluster.bucket("travel-sample");
    let collection = bucket.default_collection();

    let result = collection.get("airline_10", None).await.unwrap();
    println!("Got document: {:?}", result.content_as::<serde_json::Value>());
}
// end::tokio-main[]

pub async fn basic_await(collection: Collection) -> Result<(), ExamplesError> {
    // tag::basic-await[]
    let result = collection.get("airline_10", None).await?;
    let content = result.content_as::<serde_json::Value>()?;
    println!("Got document: {content}");
    // end::basic-await[]

    Ok(())
}

pub async fn join(collection: Collection) -> Result<(), ExamplesError> {
    // tag::join[]
    let (get_result, upsert_result) = tokio::join!(
        collection.get("airline_10", None),
        collection.upsert("airline_10", json!({"type": "airline", "name": "40-Mile Air"}), None),
    );

    match get_result {
        Ok(doc) => println!("Got: {:?}", doc.content_as::<serde_json::Value>()),
        Err(e) => println!("Get failed: {e}"),
    }
    match upsert_result {
        Ok(_) => println!("Upsert succeeded"),
        Err(e) => println!("Upsert failed: {e}"),
    }
    // end::join[]

    Ok(())
}

pub async fn join_all_example(collection: Collection) -> Result<(), ExamplesError> {
    // tag::join-all[]
    let keys = vec!["airline_10", "airline_10123", "airline_10226"];

    let futures = keys
        .iter()
        .map(|key| collection.get(*key, None));

    let results = join_all(futures).await;

    for (key, result) in keys.iter().zip(results) {
        match result {
            Ok(doc) => println!("{key}: {:?}", doc.content_as::<serde_json::Value>()),
            Err(e) => println!("{key}: error — {e}"),
        }
    }
    // end::join-all[]

    Ok(())
}

pub async fn futures_unordered(collection: Collection) -> Result<(), ExamplesError> {
    // tag::futures-unordered[]
    let keys = vec!["airline_10", "airline_10123", "airline_10226"];

    let mut unordered: FuturesUnordered<_> = keys
        .iter()
        .map(|key| {
            let col = collection.clone();
            async move { (*key, col.get(*key, None).await) }
        })
        .collect();

    while let Some((key, result)) = unordered.next().await {
        match result {
            Ok(doc) => println!("{key}: {:?}", doc.content_as::<serde_json::Value>()),
            Err(e) => println!("{key}: error — {e}"),
        }
    }
    // end::futures-unordered[]

    Ok(())
}

pub async fn spawn(collection: Collection) -> Result<(), ExamplesError> {
    // tag::spawn[]
    // SDK handles are Clone, so you can clone the handle and move the clone into the task.
    let col = collection.clone();
    let handle = tokio::spawn(async move {
        col.upsert("background-key", json!({"processed": true}), None)
            .await
    });

    // Do other work concurrently while the task runs in the background...
    collection.get("some-other-key", None).await?;

    match handle.await {
        Ok(Ok(_)) => println!("Background upsert succeeded"),
        Ok(Err(e)) => println!("Background upsert failed: {e}"),
        Err(e) => println!("Task panicked: {e}"),
    }
    // end::spawn[]

    Ok(())
}

pub async fn concurrent_errors(collection: Collection) -> Result<(), ExamplesError> {
    // tag::concurrent-errors[]
    let keys = vec!["airline_10", "airline_does_not_exist", "airline_10226"];

    let futures = keys.iter().map(|key| collection.get(*key, None));
    let results = join_all(futures).await;

    for (key, result) in keys.iter().zip(results) {
        match result {
            Ok(doc) => println!("{key}: {:?}", doc.content_as::<serde_json::Value>()),
            Err(e) => println!("{key}: {e}"),
        }
    }
    // end::concurrent-errors[]

    Ok(())
}

pub async fn try_join(collection: Collection) -> Result<(), ExamplesError> {
    // tag::try-join[]
    // try_join! returns early with the first error it encounters.
    // Here the second key does not exist, so the whole expression returns
    // a not-found error without waiting for any remaining futures.
    match tokio::try_join!(
        collection.get("airline_10", None),
        collection.get("airline_does_not_exist", None),
    ) {
        Ok((first, second)) => {
            println!("First: {:?}", first.content_as::<serde_json::Value>());
            println!("Second: {:?}", second.content_as::<serde_json::Value>());
        }
        Err(e) => println!("One of the operations failed, aborting: {e}"),
    }
    // end::try-join[]

    Ok(())
}

pub async fn timeout(collection: Collection) -> Result<(), ExamplesError> {
    // tag::timeout[]
    let result = tokio::time::timeout(
        Duration::from_millis(200),
        collection.get("airline_10", None),
    )
        .await;

    match result {
        Ok(Ok(doc)) => println!("Got: {:?}", doc.content_as::<serde_json::Value>()),
        Ok(Err(e)) => println!("Operation error: {e}"),
        Err(_elapsed) => println!("Operation timed out after 200ms"),
    }
    // end::timeout[]

    Ok(())
}

pub async fn concurrent_operations() -> Result<(), ExamplesError> {
    // tag::connect[]
    let username = "<your-username>";
    let password = "<your-password>";
    let bucket_name = "travel-sample";

    let cluster = tokio::time::timeout(
        Duration::from_secs(60),
        Cluster::connect(
            // For a secure cluster connection, use `couchbases://<your-cluster-ip>` instead.
            "couchbase://localhost",
            ClusterOptions::new(Authenticator::PasswordAuthenticator(
                PasswordAuthenticator::new(username, password),
            )),
        ),
    )
        .await.unwrap()?; // Unwrapping for brevity; handle errors as appropriate in production code.

    let bucket = cluster.bucket(bucket_name);

    tokio::time::timeout(Duration::from_secs(30), bucket.wait_until_ready(None)).await.unwrap()?;

    let collection = bucket.default_collection();
    // end::connect[]

    // tag::worker-pool[]
    // We'll create 24 worker tasks and a channel with space for a maximum of 1 item per task.
    // Writing to the channel will block if there are no tasks ready to pick up an item.
    let num_workers = 24;
    let (task_sender, task_receiver) =
        crossfire::mpmc::bounded_async::<(String, serde_json::Value)>(num_workers);

    let workers: Vec<_> = (0..num_workers)
        .map(|_| {
            let collection = collection.clone();
            let receiver = task_receiver.clone();
            tokio::spawn(async move {
                while let Ok((doc_id, value)) = receiver.recv().await {
                    if let Err(e) = collection.upsert(doc_id, value, None).await {
                        eprintln!("Upsert failed: {e}");
                    }
                }
            })
        })
        .collect();
    // end::worker-pool[]

    // tag::load-data[]
    let sample_path = format!("/opt/couchbase/samples/{bucket_name}.zip");

    // unwrap used for brevity; handle errors as appropriate in production code.
    let mut archive =
        zip::ZipArchive::new(std::fs::File::open(sample_path).unwrap()).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let file_name = file.name().to_string();

        // We only want JSON files from the docs directory.
        if file.is_dir()
            || !(file_name.starts_with(&format!("{bucket_name}/docs/"))
            && file_name.ends_with(".json"))
        {
            continue;
        }

        let mut content = String::new();
        std::io::Read::read_to_string(&mut file, &mut content).unwrap();
        let doc_content: serde_json::Value = serde_json::from_str(&content).unwrap();

        task_sender.send((file_name, doc_content)).await.unwrap();
    }
    // end::load-data[]

    // tag::wait[]
    drop(task_sender);

    futures::future::join_all(workers)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    // end::wait[]

    Ok(())
}
