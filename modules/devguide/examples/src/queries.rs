use crate::examples_error::ExamplesError;
use couchbase::authenticator::{Authenticator, PasswordAuthenticator};
use couchbase::cluster::Cluster;
use couchbase::mutation_state::MutationState;
use couchbase::options::cluster_options::ClusterOptions;
use couchbase::options::query_index_mgmt_options::{
    CreatePrimaryQueryIndexOptions, CreateQueryIndexOptions,
};
use couchbase::options::query_options::{QueryOptions, ScanConsistency};
use couchbase::scope::Scope;
use futures_util::StreamExt;
use serde::Deserialize;
use std::vec;

pub async fn simple() -> Result<(), ExamplesError> {
    // #tag::connect[]
    let cluster = Cluster::connect(
        "couchbase://localhost",
        ClusterOptions::new(Authenticator::PasswordAuthenticator(
            PasswordAuthenticator::new("username".to_string(), "password".to_string()),
        )),
    )
    .await?;
    // #end::connect[]

    // #tag::simple[]
    let scope = cluster.bucket("travel-sample").scope("inventory");
    let statement = "SELECT * from `airline` LIMIT 10;";
    let mut result = scope.query(statement, None).await?;

    let mut rows = result.rows();
    while let Some(row) = rows.next().await {
        let row: serde_json::Value = row?;
        println!("Row: {}", row);
    }
    // #end::simple[]

    Ok(())
}

pub async fn adhoc_query(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::adhocquery[]
    let result = cluster
        .bucket("travel-sample")
        .scope("inventory")
        .query(
            "SELECT count(*) from `airport`",
            QueryOptions::new().ad_hoc(false),
        )
        .await?;
    // #end::adhocquery[]

    Ok(())
}

pub async fn request_plus_query(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::request-plus[]
    let result = cluster
        .bucket("travel-sample")
        .scope("inventory")
        .query(
            "SELECT count(*) from `airport`",
            QueryOptions::new().scan_consistency(ScanConsistency::RequestPlus),
        )
        .await?;
    // #end::request-plus[]

    Ok(())
}

pub async fn at_plus_query(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::atplus[]
    let mutation_state = {
        let result = cluster
            .bucket("travel-sample")
            .scope("inventory")
            .collection("airport")
            .upsert(
                "airport_1254",
                serde_json::json!({"name": "New Airport"}),
                None,
            )
            .await?;

        // MutationState can be created from a token directly.
        let state = MutationState::from(result.mutation_token().clone().unwrap());

        state
    };

    let result = cluster
        .bucket("travel-sample")
        .scope("inventory")
        .query(
            "SELECT count(*) from `airport`",
            QueryOptions::new().scan_consistency(ScanConsistency::AtPlus(mutation_state)),
        )
        .await?;
    // #end::atplus[]

    Ok(())
}

pub async fn create_indexes(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::createindexes[]
    let index_manager = cluster
        .bucket("travel-sample")
        .scope("index")
        .collection("users")
        .query_indexes();

    index_manager.create_primary_index(None).await?;

    index_manager
        .create_index("ix_name", vec!["name".to_string()], None)
        .await?;

    index_manager
        .create_index("ix_email", vec!["preferred_email".to_string()], None)
        .await?;
    // #end::createindexes[]

    Ok(())
}

pub async fn deferred_indexes(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::deferredindexes[]
    let index_manager = cluster
        .bucket("travel-sample")
        .scope("index")
        .collection("users")
        .query_indexes();

    index_manager
        .create_primary_index(CreatePrimaryQueryIndexOptions::new().deferred(true))
        .await?;

    index_manager
        .create_index(
            "ix_name",
            vec!["name".to_string()],
            CreateQueryIndexOptions::new().deferred(true),
        )
        .await?;

    index_manager
        .create_index(
            "ix_email",
            vec!["preferred_email".to_string()],
            CreateQueryIndexOptions::new().deferred(true),
        )
        .await?;

    index_manager.build_deferred_indexes(None).await?;

    index_manager
        .watch_indexes(
            vec![
                "#primary".to_string(),
                "ix_name".to_string(),
                "ix_email".to_string(),
            ],
            None,
        )
        .await?;
    // #end::deferredindexes[]

    Ok(())
}

pub async fn errors(scope: Scope) -> Result<(), ExamplesError> {
    // #tag::errors[]
    let statement = "SELECT * from `airport` LIMIT 10;";
    match scope.query(statement, None).await {
        Ok(res) => {
            println!("Success!");
        }
        Err(e) => {
            println!("Query failed: {e}");
        }
    };
    // #end::errors[]

    Ok(())
}

pub async fn positional_params(scope: Scope) -> Result<(), ExamplesError> {
    // #tag::positionalparams[]
    let statement = "SELECT * from `airline` WHERE country = $1;";
    let mut result = scope
        .query(
            statement,
            QueryOptions::new().add_positional_parameter("United States")?,
        )
        .await?;
    // #end::positionalparams[]

    Ok(())
}

pub async fn named_params(scope: Scope) -> Result<(), ExamplesError> {
    // #tag::namedparams[]
    let statement = "SELECT * from `airline` WHERE country = $country;";
    let mut result = scope
        .query(
            statement,
            QueryOptions::new().add_named_parameter("country", "United States")?,
        )
        .await?;
    // #end::namedparams[]

    Ok(())
}

// #tag::deserialization[]
#[derive(Deserialize)]
pub struct Address {
    city: String,
}

#[derive(Deserialize)]
pub struct User {
    name: String,
    address: Vec<Address>,
    age: u8,
}
// #end::deserialization[]

pub async fn own_type(scope: Scope) -> Result<(), ExamplesError> {
    // #tag::owntype[]
    let statement = "SELECT name, address, age from `users` LIMIT 10;";
    let mut result = scope.query(statement, None).await?;

    let mut rows = result.rows();
    while let Some(row) = rows.next().await {
        let user: User = row?;
        println!("User: {} in city {}", user.name, user.address[0].city);
    }
    // #end::owntype[]

    Ok(())
}

pub async fn cluster_level_query(cluster: Cluster) -> Result<(), ExamplesError> {
    // #tag::clusterlevelquery[]
    let statement = "SELECT name from `travel-sample` LIMIT 10;";
    let mut result = cluster.query(statement, None).await?;
    // #end::clusterlevelquery[]

    Ok(())
}
