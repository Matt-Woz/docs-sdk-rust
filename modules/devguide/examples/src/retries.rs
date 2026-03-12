use couchbase::authenticator::{Authenticator, PasswordAuthenticator};
use couchbase::options::cluster_options::ClusterOptions;
use couchbase::options::kv_options::{InsertOptions, UpsertOptions};
use couchbase::retry::{
    BestEffortRetryStrategy, ExponentialBackoffCalculator, RetryAction, RetryReason, RetryRequest,
    RetryStrategy,
};
use std::sync::Arc;

pub async fn global_retry_strategy() {
    // #tag::global[]
    let opts = ClusterOptions::new(Authenticator::PasswordAuthenticator(
        PasswordAuthenticator::new("username".to_string(), "password".to_string()),
    ))
    .default_retry_strategy(Arc::new(BestEffortRetryStrategy::new(
        ExponentialBackoffCalculator::default(),
    )));
    // #end::global[]
}

pub async fn request_retry_strategy() {
    // #tag::request[]
    let opts = UpsertOptions::new().retry_strategy(Arc::new(BestEffortRetryStrategy::new(
        ExponentialBackoffCalculator::default(),
    )));
    // #end::request[]
}

pub async fn shared_retry_strategy() {
    // #tag::shared[]
    let retry_strategy = Arc::new(BestEffortRetryStrategy::new(
        ExponentialBackoffCalculator::default(),
    ));
    let opts = UpsertOptions::new().retry_strategy(retry_strategy.clone());
    let opts2 = InsertOptions::new().retry_strategy(retry_strategy.clone());
    // #end::shared[]
}

pub async fn custom_retry_strategy() {
    // #tag::custom[]
    #[derive(Debug)]
    struct CustomRetryStrategy {
        base_strategy: BestEffortRetryStrategy<ExponentialBackoffCalculator>,
    }

    impl RetryStrategy for CustomRetryStrategy {
        fn retry_after(&self, request: &RetryRequest, reason: &RetryReason) -> Option<RetryAction> {
            match reason {
                RetryReason::KvLocked => {
                    // Override the default and don't retry.
                    None
                }
                _ => self.base_strategy.retry_after(request, reason),
            }
        }
    }

    let base_strategy = BestEffortRetryStrategy::new(ExponentialBackoffCalculator::default());

    let retry_strategy = Arc::new(CustomRetryStrategy { base_strategy });
    let opts = UpsertOptions::new().retry_strategy(retry_strategy.clone());
    // #end::custom[]
}
