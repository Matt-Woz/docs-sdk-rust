use crate::examples_error::ExamplesError;
use crate::timeout::timeout;
use couchbase::authenticator::{Authenticator, PasswordAuthenticator};
use couchbase::cluster::Cluster;
use couchbase::options::cluster_options::{ClusterOptions, TlsOptions};
use std::time::Duration;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::CertificateDer;

pub async fn connect() {
    // tag::connect[]
    // Update this to your cluster
    let username = "<your-username>";
    let password = "<your-password>";
    let bucket_name = "travel-sample";

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
    .await
    .unwrap();
    // end::connect[]
}

pub async fn multiple_endpoints() -> Result<(), ExamplesError> {
    let username = "<your-username>";
    let password = "<your-password>";
    // tag::multiple-endpoints[]
    let connection_string = "couchbase://node1.example.com,node2.example.com,node3.example.com";
    let cluster = Cluster::connect(
        connection_string,
        ClusterOptions::new(Authenticator::PasswordAuthenticator(
            PasswordAuthenticator::new(username, password),
        )),
    )
    .await?;
    // end::multiple-endpoints[]

    Ok(())
}

pub async fn bucket_wait_until_ready(cluster: Cluster) -> Result<(), ExamplesError> {
    // tag::bucket-wait-until-ready[]
    let bucket = cluster.bucket("travel-sample");
    timeout(Duration::from_secs(30), bucket.wait_until_ready(None)).await?;
    // end::bucket-wait-until-ready[]

    Ok(())
}

pub async fn secure_connection() -> Result<(), ExamplesError> {
    let username = "<your-username>";
    let password = "<your-password>";
    // tag::secure-connection[]
    // Unwrap used for brevity; handle errors as appropriate in production code.
    let ca_cert = CertificateDer::from_pem_file("/path/to/cluster-root-certificate.pem").unwrap();

    let cluster = Cluster::connect(
        "couchbases://node1.example.com",
        ClusterOptions::new(Authenticator::PasswordAuthenticator(
            PasswordAuthenticator::new(username, password),
        ))
        .tls_options(TlsOptions::new().add_ca_certificate(ca_cert)),
    )
    .await?;
    // end::secure-connection[]

    Ok(())
}

pub async fn insecure_connection() -> Result<(), ExamplesError> {
    let username = "<your-username>";
    let password = "<your-password>";
    // tag::insecure-connection[]
    let cluster = Cluster::connect(
        "couchbases://node1.example.com",
        ClusterOptions::new(Authenticator::PasswordAuthenticator(
            PasswordAuthenticator::new(username, password),
        ))
        .tls_options(TlsOptions::new().danger_accept_invalid_certs(true)),
    )
    .await?;
    // end::insecure-connection[]

    Ok(())
}
