use couchbase::bucket::Bucket;
use couchbase::cluster::Cluster;
use couchbase::management::buckets::bucket_settings::{
    BucketSettings, BucketType, ConflictResolutionType,
};
use couchbase::management::collections::collection_manager::CreateCollectionSettings;
use couchbase::management::collections::collection_settings::MaxExpiryValue;
use couchbase::management::users::user::{Role, User};
use couchbase::options::query_index_mgmt_options::{
    CreatePrimaryQueryIndexOptions, CreateQueryIndexOptions,
};

async fn bucket_manager(cluster: Cluster) {
    // #tag::bucket[]
    let bucket_manager = cluster.buckets();

    let bucket = BucketSettings::new("hello")
        .flush_enabled(false)
        .replica_indexes(false)
        .num_replicas(1)
        .bucket_type(BucketType::COUCHBASE)
        .conflict_resolution_type(ConflictResolutionType::SEQUENCE_NUMBER);

    match bucket_manager.create_bucket(bucket, None).await {
        Ok(_) => println!("Bucket created successfully"),
        Err(e) => println!("Error: {e}"),
    }
    // #end::bucket[]

    // #tag::flush[]
    match bucket_manager.get_bucket("hello", None).await {
        Ok(bucket) => {
            let bucket = bucket.flush_enabled(true);
            match bucket_manager.update_bucket(bucket, None).await {
                Ok(_) => println!("Bucket updated successfully"),
                Err(e) => println!("Error: {e}"),
            }
        }
        Err(e) => println!("Error: {e}"),
    }
    // #end::flush[]

    // #tag::drop[]
    match bucket_manager.drop_bucket("hello", None).await {
        Ok(_) => println!("Bucket dropped successfully"),
        Err(e) => println!("Error: {e}"),
    }
    // #end::drop[]

    // #tag::flush[]
    match bucket_manager.flush_bucket("hello", None).await {
        Ok(_) => println!("Bucket flushed successfully"),
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::BucketNotFlushable => {
                println!("Flushing is not enabled on this bucket")
            }
            _ => {
                println!("Error: {e}")
            }
        },
    }
    // #end::flush[]
}

async fn collection_manager(bucket: Bucket) {
    // #tag::collections[]
    let collection_manager = bucket.collections();
    // #end::collections[]

    // #tag::create_scope[]
    match collection_manager.create_scope("my_scope", None).await {
        Ok(_) => println!("Scope created successfully"),
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::ScopeExists => {
                println!("Scope already exists");
            }
            _ => {
                println!("Error: {e}");
            }
        },
    }
    // #end::create_scope[]

    // #tag::create_collection[]
    let settings = CreateCollectionSettings::new()
        .max_expiry(MaxExpiryValue::InheritFromBucket)
        .history(false);

    match collection_manager
        .create_collection("example-scope", "example_collection", settings, None)
        .await
    {
        Ok(_) => println!("Collection created successfully"),
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::ScopeNotFound => {
                println!("Scope does not exist");
            }
            couchbase::error::ErrorKind::CollectionExists => {
                println!("Collection already exists");
            }
            _ => {
                println!("Error: {e}");
            }
        },
    }
    // #end::create_collection[]

    // #tag::drop_collection[]
    match collection_manager
        .drop_collection("example-scope", "example-collection", None)
        .await
    {
        Ok(_) => println!("Collection dropped successfully"),
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::ScopeNotFound => {
                println!("Scope not found");
            }
            couchbase::error::ErrorKind::CollectionNotFound => {
                println!("Collection not found");
            }
            _ => {
                println!("Error: {e}");
            }
        },
    }
    // #end::drop_collection[]

    // #tag::drop_scope[]
    match collection_manager.drop_scope("example-scope", None).await {
        Ok(_) => println!("Scope dropped successfully"),
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::ScopeNotFound => {
                println!("Scope not found");
            }
            _ => {
                println!("Error: {e}");
            }
        },
    }
    // #end::drop_scope[]
}

async fn user_manager(cluster: Cluster) {
    // #tag::scope_admin[]
    let user_manager = cluster.users();
    let user = User::new(
        "scope-admin",
        "display-name",
        vec![
            Role::new("scope_admin").bucket("travel-sample"),
            Role::new("data_reader").bucket("travel-sample"),
        ],
    );
    match user_manager
        .upsert_user(user.password("password"), None)
        .await
    {
        Ok(_) => println!("User created successfully"),
        Err(e) => println!("Error: {e}"),
    }
    // #end::scope_admin[]
}

async fn query_index_manager(cluster: Cluster) {
    // #tag::query_index_manager[]
    let collection = cluster
        .bucket("travel-sample")
        .scope("tenant_agent_01")
        .collection("users");
    let query_index_manager = collection.query_indexes();
    // #end::query_index_manager[]

    // #tag::create_primary_index[]
    match query_index_manager
        .create_primary_index(
            CreatePrimaryQueryIndexOptions::new()
                .index_name("custom_name")
                .ignore_if_exists(true),
        )
        .await
    {
        Ok(_) => println!("Primary index created successfully"),
        Err(e) => println!("Error: {e}"),
    }
    // #end::create_primary_index[]

    // #tag::create_secondary_index[]
    match query_index_manager
        .create_index("tenant_agent_01_users_email", vec!["preferred_email".to_string()], None)
        .await
    {
        Ok(_) => println!("Secondary index created successfully"),
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::IndexExists => {
                println!("Index already exists");
            }
            _ => {
                println!("Error: {e}");
            }
        },
    }
    // #end::create_secondary_index[]

    // #tag::deferred_indexes[]
    match query_index_manager
        .create_index(
            "tenant_agent_01_users_email",
            vec!["preferred_email".to_string()],
            CreateQueryIndexOptions::new().deferred(true),
        )
        .await
    {
        Ok(_) => println!("Secondary index created successfully"),
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::IndexExists => {
                println!("Index already exists");
            }
            _ => {
                println!("Error: {e}");
            }
        },
    }

    match query_index_manager.build_deferred_indexes(None).await {
        Ok(_) => println!("Deferred indexes are being built"),
        Err(e) => println!("Error: {e}"),
    }

    match query_index_manager
        .watch_indexes(vec!["tenant_agent_01_users_phone".to_string()], None)
        .await
    {
        Ok(_) => println!("All watched indexes are online"),
        Err(e) => println!("Error: {e}"),
    }
    // #end::deferred_indexes[]

    // #tag::drop_indexes[]
    match query_index_manager.drop_primary_index(None).await {
        Ok(_) => println!("Primary index dropped successfully"),
        Err(e) => println!("Error: {e}"),
    }

    match query_index_manager
        .drop_index("tenant_agent_01_users_email", None)
        .await
    {
        Ok(_) => println!("Secondary index dropped successfully"),
        Err(e) => println!("Error: {e}"),
    }
    // #end::drop_indexes[]
}
