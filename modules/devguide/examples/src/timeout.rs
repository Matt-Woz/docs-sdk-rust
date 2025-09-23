use std::time::Duration;

pub async fn timeout<F, R>(duration: Duration, fut: F) -> Result<R, couchbase::error::Error>
where
    F: Future<Output = Result<R, couchbase::error::Error>>,
{
    tokio::time::timeout(duration, fut).await.unwrap()
}
