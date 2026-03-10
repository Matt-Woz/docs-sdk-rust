use crate::examples_error::ExamplesError;
use crate::timeout::timeout;
use couchbase::authenticator::{Authenticator, PasswordAuthenticator};
use couchbase::cluster::Cluster;
use couchbase::options::cluster_options::ClusterOptions;
use couchbase::options::diagnostic_options::{PingOptions, WaitUntilReadyOptions};
use couchbase::service_type::ServiceType;
use std::time::Duration;

pub async fn ping(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::ping[]
    let result = cluster
        .ping(PingOptions::new().service_types(vec![ServiceType::KV, ServiceType::QUERY]))
        .await?;

    println!("Ping result: {result}");
    /*
    {
      "config_rev": 939408,
      "id": "f3af8565-197d-49f6-8248-ce59ebf585fb",
      "sdk": "rust",
      "services": {
        "Kv": [
          {
            "id": "e8ab555c-b561-4d79-8653-b694434cab4f",
            "latency_us": 294,
            "remote": "192.168.107.130:11207",
            "state": "ok"
          },
          {
            "id": "54362aa1-83a6-4514-9e13-579a43534649",
            "latency_us": 309,
            "remote": "192.168.107.129:11207",
            "state": "ok"
          },
          {
            "id": "67620757-f549-4cf9-a11a-188bee40bb98",
            "latency_us": 269,
            "remote": "192.168.107.128:11207",
            "state": "ok"
          }
        ],
        "Query": [
          {
            "latency_us": 6103,
            "remote": "https://192.168.107.130:18093",
            "state": "ok"
          },
          {
            "latency_us": 8512,
            "remote": "https://192.168.107.129:18093",
            "state": "ok"
          },
          {
            "latency_us": 6549,
            "remote": "https://192.168.107.128:18093",
            "state": "ok"
          }
        ]
      },
      "version": 2
    }
         */
    // #end::ping[]

    Ok(())
}

pub async fn diagnostics(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::diagnostics[]
    let result = cluster.diagnostics(None).await?;

    println!("Diagnostics result: {result}");
    /*
    {
      "version": 2,
      "config_rev": 8101,
      "id": "a84ef919-abc2-4dd2-8c4d-2220cc645d7c",
      "sdk": "rust",
      "services": {
        "Kv": [
          {
            "service_type": "Kv",
            "id": "701259c9-d43c-4898-832a-f62e2934014b",
            "local_address": "192.168.106.1:60774",
            "remote_address": "192.168.106.128:11210",
            "last_activity": 2395,
            "state": "Connected"
          },
          {
            "service_type": "Kv",
            "id": "c7c24770-0746-4493-b0d3-b5013de48bf4",
            "local_address": "192.168.106.1:60775",
            "remote_address": "192.168.106.129:11210",
            "last_activity": 9495,
            "state": "Connected"
          },
          {
            "service_type": "Kv",
            "id": "fc883157-95f6-409a-8795-0776e788e6db",
            "local_address": "192.168.106.1:60773",
            "remote_address": "192.168.106.130:11210",
            "last_activity": 357556,
            "state": "Connected"
          }
        ]
  }
  */
    // #end::diagnostics[]

    Ok(())
}

pub async fn wait_until_ready() -> Result<(), ExamplesError> {
    // #tag::cluster-wait-until-ready[]
    let username = "<your-username>";
    let password = "<your-password>";

    let cluster = timeout(
        Duration::from_secs(60),
        Cluster::connect(
            // For a secure cluster connection, use `couchbases://<your-cluster-ip>` instead.
            "couchbase://localhost",
            ClusterOptions::new(Authenticator::PasswordAuthenticator(
                PasswordAuthenticator::new(username, password),
            )),
        ),
    )
    .await?;

    cluster
        .wait_until_ready(WaitUntilReadyOptions::default())
        .await?;
    // #end::cluster-wait-until-ready[]

    Ok(())
}
