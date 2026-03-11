use crate::examples_error::ExamplesError;
use couchbase::authenticator::{Authenticator, PasswordAuthenticator};
use couchbase::cluster::Cluster;
use couchbase::collection::Collection;
use couchbase::options::cluster_options::ClusterOptions;
use futures::future::join_all;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;
use std::time::Duration;

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
    let (first, second, third) = tokio::join!(
        collection.get("airline_10", None),
        collection.get("airline_10123", None),
        collection.get("airline_10226", None),
    );

    for result in [first, second, third] {
        match result {
            Ok(doc) => println!("Got: {:?}", doc.content_as::<serde_json::Value>()),
            Err(e) => println!("Error: {e}"),
        }
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
    // SDK handles are Clone, so they can be moved into spawned tasks.
    let handle = tokio::spawn(async move {
        collection.upsert("background-key", json!({"processed": true}), None)
            .await
    });

    // Do other work concurrently while the task runs in the background...

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

