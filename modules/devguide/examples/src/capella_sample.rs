use crate::examples_error::ExamplesError;
use crate::timeout::timeout;
use couchbase::authenticator::{Authenticator, PasswordAuthenticator};
use couchbase::cluster::Cluster;
use couchbase::durability_level::DurabilityLevel;
use couchbase::options::cluster_options::ClusterOptions;
use couchbase::options::kv_options::ReplaceOptions;
use serde_json::json;
use std::time::Duration;

pub async fn capella_sample() -> Result<(), ExamplesError> {
    // tag::connect[]
    // Update this to your cluster
    let endpoint = "cb.<your-endpoint>.cloud.couchbase.com";
    let username = "<your-username>";
    let password = "<your-password>";
    let bucket_name = "travel-sample";

    let cluster_options = ClusterOptions::new(Authenticator::PasswordAuthenticator(
        PasswordAuthenticator::new(username.to_string(), password.to_string()),
    ));

    let cluster = timeout(
        Duration::from_secs(30),
        Cluster::connect(format!("couchbases://{endpoint}"), cluster_options),
    )
    .await
    .unwrap();
    // end::connect[]

    // tag::bucket[]
    let bucket = cluster.bucket(bucket_name);
    timeout(Duration::from_secs(30), bucket.wait_until_ready(None))
        .await
        .unwrap();
    // end::bucket[]

    // tag::collection[]
    let collection = bucket.scope("inventory").collection("airport");
    // end::collection[]

    // tag::json[]
    let doc = json!({"status": "awesome"});
    // end::json[]

    // tag::upsert[]
    let doc_id = uuid::Uuid::new_v4().to_string();
    timeout(
        Duration::from_millis(2500),
        collection.upsert(&doc_id, doc, None),
    )
    .await
    .unwrap();
    // end::upsert[]

    // tag::get[]
    let result = collection.get(&doc_id, None).await.unwrap();
    let value: serde_json::Value = result.content_as().unwrap();
    let status = value.get("status").unwrap();
    let status = status.as_str().unwrap();

    println!("Couchbase is ${status}");
    // end::get[]

    // tag::get-better-error-handling-propagate[]
    let get_result = collection.get(&doc_id, None).await?;
    let value: serde_json::Value = get_result.content_as()?;

    // value.get returns an Options, so we unwrap it here for simplicity.
    let status = value.get("status").unwrap();
    let status = status.as_str().unwrap();

    println!("Couchbase is ${status}");
    //end::get-better-error-handling-propagate[]

    // tag::get-better-error-handling[]
    match collection
        .get(&doc_id, None)
        .await
        .and_then(|result| result.content_as::<serde_json::Value>())
    {
        Ok(value) => {
            // We could convert the Option from `get` into a Result and include it in the function chain
            // above if we wanted to.
            let status = value.get("status").unwrap();
            let status = status.as_str().unwrap();

            println!("Couchbase is ${status}")
        }
        Err(e) => println!("Error: {e}"),
    };
    //end::get-better-error-handling[]

    // tag::replace-options[]
    timeout(
        Duration::from_millis(2500),
        collection.replace(
            &doc_id,
            value,
            ReplaceOptions::new()
                .expiry(Duration::from_secs(10))
                .durability_level(DurabilityLevel::MAJORITY),
        ),
    )
    .await
    .unwrap();
    // end::replace-options[]

    Ok(())
}
