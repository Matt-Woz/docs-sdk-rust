use couchbase::authenticator::{Authenticator, PasswordAuthenticator};
use couchbase::options::cluster_options::{ClusterOptions, TlsOptions};

pub fn security() {
    // #tag::security[]
    ClusterOptions::new(Authenticator::PasswordAuthenticator(
        PasswordAuthenticator::new("username".to_string(), "password".to_string()),
    ))
    .tls_options(TlsOptions::new());
    // #end::security[]
}
