use crate::examples_error::ExamplesError;
use couchbase::collection::Collection;
use couchbase::transcoding::{raw_binary, raw_json, raw_string};
use serde::de::DeserializeOwned;

pub async fn raw_json_transcoder(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::raw_json_transcoder[]
    let value = r#"{"type":"raw_json","name":"Raw JSON Example"}"#;

    // This effectively does nothing more than convert &str to &[u8] and set the flags to indicate JSON.
    let (encoded, flags) = raw_json::encode(&value)?;

    collection
        .upsert_raw("doc-id", encoded, flags, None)
        .await?;

    let doc = collection.get("doc-id", None).await?;
    let (content_raw, flags) = doc.content_as_raw();

    // value will be of type &[u8]
    let value = raw_json::decode(content_raw, flags)?;
    // #end::raw_json_transcoder[]

    Ok(())
}

pub async fn raw_string_transcoder(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::raw_string_transcoder[]
    let value = "This is a raw string";

    // This effectively does nothing more than convert &str to &[u8] and set the flags to indicate a raw string.
    let (encoded, flags) = raw_string::encode(&value)?;

    collection
        .upsert_raw("doc-id", encoded, flags, None)
        .await?;

    let doc = collection.get("doc-id", None).await?;
    let (content_raw, flags) = doc.content_as_raw();

    // value will be of type &str
    let value = raw_string::decode(content_raw, flags)?;
    // #end::raw_string_transcoder[]

    Ok(())
}

pub async fn raw_binary_transcoder(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::raw_binary_transcoder[]
    let value: &[u8] = b"This is raw binary";

    // This effectively does nothing more than take &[u8] and set the flags to indicate binary data.
    let (encoded, flags) = raw_binary::encode(&value)?;

    collection
        .upsert_raw("doc-id", encoded, flags, None)
        .await?;

    let doc = collection.get("doc-id", None).await?;
    let (content_raw, _flags) = doc.content_as_raw();

    // value will be of type &[u8]
    let value = raw_binary::decode(content_raw, flags)?;
    // #end::raw_binary_transcoder[]

    Ok(())
}

// #tag::msgpack[]
pub fn encode_msg_pack<T>(value: T) -> Result<(Vec<u8>, u32), rmp_serde::encode::Error>
where
    T: serde::Serialize,
{
    let encoded = rmp_serde::to_vec(&value)?;
    Ok((
        encoded,
        couchbase::transcoding::encode_common_flags(couchbase::transcoding::DataType::Binary),
    ))
}

pub fn decode_msg_pack<T>(encoded: &[u8], _flags: u32) -> Result<T, rmp_serde::decode::Error>
where
    T: DeserializeOwned,
{
    rmp_serde::from_slice(encoded)
}
// #end::msgpack[]

pub async fn use_msgpack_transcoder(collection: Collection) -> Result<(), ExamplesError> {
    // #tag::use_msgpack[]
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct User {
        field1: String,
        field2: i32,
    }

    let user = User {
        field1: "value1".to_string(),
        field2: 42,
    };

    // Unwrap for simplicity in this example; production code should handle the error.
    let (encoded, flags) = encode_msg_pack(&user).unwrap();

    collection
        .upsert_raw("john-smith", &encoded, flags, None)
        .await?;

    let doc = collection.get("doc-id", None).await?;
    let (content_raw, flags) = doc.content_as_raw();

    // Unwrap for simplicity in this example; production code should handle the error.
    let decoded: User = decode_msg_pack(content_raw, flags).unwrap();

    assert_eq!(user, decoded);
    // #end::use_msgpack[]

    Ok(())
}
