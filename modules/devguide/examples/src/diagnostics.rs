use crate::examples_error::ExamplesError;
use couchbase::cluster::Cluster;
use couchbase::options::diagnostic_options::PingOptions;
use couchbase::service_type::ServiceType;

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
