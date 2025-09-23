use crate::examples_error::ExamplesError;
use couchbase::collection::Collection;
use couchbase::durability_level::DurabilityLevel;
use couchbase::options::kv_options::MutateInOptions;
use couchbase::subdoc::lookup_in_specs::{GetSpecOptions, LookupInSpec};
use couchbase::subdoc::macros::LookupInMacros;
use couchbase::subdoc::mutate_in_specs::{ArrayAppendSpecOptions, MutateInSpec, UpsertSpecOptions};
use serde_json::json;

pub async fn upsert(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::upsert[]
    let result = collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::upsert("email", "dougr96@hotmail.com", None)?],
            None,
        )
        .await;

    match result {
        Ok(res) => {
            println!("Success!");
        }
        Err(e) => {
            println!("error: {e}")
        }
    }
    // #end::upsert[]

    Ok(())
}

pub async fn insert(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::insert[]
    let result = collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::insert("email", "dougr96@hotmail.com", None)?],
            None,
        )
        .await;

    match result {
        Ok(res) => {
            println!("Success!");
        }
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::PathExists => {
                println!("The field already exists.");
            }
            _ => {
                println!("error: {e}")
            }
        },
    }
    // #end::insert[]

    Ok(())
}

pub async fn combine(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::combine[]
    let result = {
        match collection
            .lookup_in(
                "customer123",
                &[
                    LookupInSpec::get("addresses.delivery.country", None),
                    LookupInSpec::exists("addresses.delivery.country", None),
                ],
                None,
            )
            .await
        {
            Ok(res) => {
                let country = res.content_as::<String>(0)?;
                let exists = res.content_as::<bool>(1)?;
                Ok((country, exists))
            }
            Err(e) => Err(e),
        }
    };

    match result {
        Ok((country, exists)) => {
            println!("Country: {country}, Exists: {exists}");
        }
        Err(e) => {
            println!("error: {e}")
        }
    }
    // #end::combine[]

    Ok(())
}

pub async fn combine_mutate(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::combine_mutate[]
    let result = collection
        .mutate_in(
            "customer123",
            &[
                MutateInSpec::remove("addresses.billing", None),
                MutateInSpec::replace("email", "dougr96@hotmail.com", None)?,
            ],
            None,
        )
        .await;
    // #end::combine_mutate[]

    Ok(())
}

pub async fn vattr_expiry(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::expiry[]
    let result = {
        collection
            .lookup_in(
                "doc-id",
                &[LookupInSpec::get(
                    LookupInMacros::ExpiryTime,
                    GetSpecOptions::new().xattr(true),
                )],
                None,
            )
            .await?
    };
    // #end::expiry[]

    Ok(())
}

pub async fn get(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::get[]
    let result = collection
        .lookup_in(
            "customer123",
            &[LookupInSpec::get("addresses.delivery.country", None)],
            None,
        )
        .await?;

    let country: String = result.content_as(0)?;
    println!("Country: {country}");
    // #end::get[]

    Ok(())
}

pub async fn exists(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::exists[]
    let result = collection
        .lookup_in(
            "customer123",
            &[LookupInSpec::exists(
                "addresses.delivery.does_not_exist",
                None,
            )],
            None,
        )
        .await?;

    let exists: bool = result.content_as(0)?;
    println!("Does field exist? {exists}");
    // #end::exists[]

    Ok(())
}

pub async fn array_append(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::array_append[]
    let result = collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::array_append(
                "purchases.complete",
                &[777],
                None,
            )?],
            None,
        )
        .await;
    // #end::array_append[]

    // purchases.complete is now [339, 976, 442, 666, 777]

    Ok(())
}

pub async fn array_prepend(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::array_prepend[]
    let result = collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::array_prepend(
                "purchases.abandoned",
                &[18],
                None,
            )?],
            None,
        )
        .await;
    // #end::array_prepend[]

    // purchases.abandoned is now [18, 157, 49, 999]

    Ok(())
}

pub async fn array_document(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::array_document[]
    let result = collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::array_append("", &["some element"], None)?],
            None,
        )
        .await;
    // #end::array_document[]

    // the document my_array is now ["some element"]

    Ok(())
}

pub async fn array_upsert(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::array_upsert[]
    let result = collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::array_append(
                "some.array",
                &["hello world"],
                ArrayAppendSpecOptions::new().create_path(true),
            )?],
            None,
        )
        .await;
    // #end::array_upsert[]

    Ok(())
}

pub async fn array_add_unique(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::array_add_unique[]
    collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::array_add_unique(
                "purchases.complete",
                95,
                None,
            )?],
            None,
        )
        .await?;

    match collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::array_add_unique(
                "purchases.complete",
                95,
                None,
            )?],
            None,
        )
        .await
    {
        Ok(_) => println!("Added 95 to the array"),
        Err(e) => match e.kind() {
            couchbase::error::ErrorKind::PathExists => {
                println!("Path already exists");
            }
            _ => {
                println!("error: {e}")
            }
        },
    }
    // #end::array_add_unique[]

    Ok(())
}

pub async fn array_insert(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::array_insert[]
    let result = collection
        .mutate_in(
            "some_doc",
            &[MutateInSpec::array_insert("foo.bar[1]", &["cruel"], None)?],
            None,
        )
        .await;
    // #end::array_insert[]

    Ok(())
}

pub async fn counter_increment(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::counter_increment[]
    let result = collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::increment("logins", 1, None)?],
            None,
        )
        .await?;

    let new_value: i64 = result.content_as(0)?;
    println!("New age: {new_value}");
    // #end::counter_increment[]

    Ok(())
}

pub async fn counter_decrement(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::counter_decrement[]
    collection
        .upsert("player432", json!({"gold": 1000}), None)
        .await?;

    let result = collection
        .mutate_in(
            "player432",
            &[MutateInSpec::decrement("gold", 150, None)?],
            None,
        )
        .await?;

    let new_value: i64 = result.content_as(0)?;
    println!("New gold amount: {new_value}");
    // #end::counter_decrement[]

    Ok(())
}

pub async fn create_path(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::create_path[]
    let result = collection
        .mutate_in(
            "customer123",
            &[MutateInSpec::upsert(
                "level_0.level_1.foo.bar.phone",
                json!({"num": "311-555-0101", "ext": 16}),
                UpsertSpecOptions::new().create_path(true),
            )?],
            None,
        )
        .await;
    // #end::create_path[]

    Ok(())
}

pub async fn concurrent_subdoc_operations(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::concurrent_subdoc_operations[]
    let collection1 = collection.clone();
    let handle1 = tokio::spawn(async move {
        collection1
            .mutate_in(
                "customer123",
                &[MutateInSpec::array_append(
                    "purchases.complete",
                    &[99],
                    None,
                )?],
                None,
            )
            .await
    });

    let handle2 = tokio::spawn(async move {
        collection
            .mutate_in(
                "customer123",
                &[MutateInSpec::array_append(
                    "purchases.abandoned",
                    &[101],
                    None,
                )?],
                None,
            )
            .await
    });

    // These are spawned tasks, so they run concurrently even though we await in serial.
    // We use unwrap for brevity, it only applies here if the task itself panicked.
    handle1.await.unwrap()?;
    handle2.await.unwrap()?;
    // #end::concurrent_subdoc_operations[]

    Ok(())
}

pub async fn cas(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::cas[]
    collection
        .mutate_in(
            "player432",
            &[MutateInSpec::decrement("gold", 150, None)?],
            MutateInOptions::new().cas(12345),
        )
        .await?;
    // #end::cas[]

    Ok(())
}

pub async fn durability(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::durability[]
    collection
        .mutate_in(
            "player432",
            &[MutateInSpec::decrement("gold", 150, None)?],
            MutateInOptions::new().durability_level(DurabilityLevel::MAJORITY),
        )
        .await?;
    // #end::durability[]

    Ok(())
}
