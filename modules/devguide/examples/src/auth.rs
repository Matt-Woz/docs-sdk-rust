use crate::examples_error::ExamplesError;
use couchbase::authenticator::{Authenticator, CertificateAuthenticator, PasswordAuthenticator};
use couchbase::cluster::Cluster;
use couchbase::options::cluster_options::{ClusterOptions, TlsOptions};
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub async fn basic() -> Result<(), ExamplesError> {
    // tag::basic[]
    Cluster::connect(
        "couchbase://10.112.180.101",
        ClusterOptions::new(Authenticator::PasswordAuthenticator(
            PasswordAuthenticator::new("username".to_string(), "password".to_string()),
        )),
    )
    .await?;
    // end::basic[]

    Ok(())
}

async fn certificate() -> Result<(), ExamplesError> {
    // tag::certificate[]
    // Unwrap used for brevity; handle errors as appropriate in production code.
    // Load in the client certificate, key, and root CA.
    let root_cert = CertificateDer::from_pem_file("/path/to/cluster-root-certificate.pem").unwrap();
    let client_certs = CertificateDer::pem_file_iter("/path/to/client-certificate.pem")
        .unwrap()
        .map(|c| c.unwrap()) // Each PEM section could fail to parse, handle errors as appropriate.
        .collect();
    let client_key = PrivateKeyDer::from_pem_file("/path/to/client-key.pem").unwrap();

    // Create the authenticator.
    let authenticator = CertificateAuthenticator::new(client_certs, client_key);

    Cluster::connect(
        "couchbases://10.112.180.101",
        ClusterOptions::new(Authenticator::CertificateAuthenticator(authenticator))
            .tls_options(TlsOptions::new().add_ca_certificate(root_cert)),
    )
    .await?;
    // end::certificate[]

    Ok(())
}
