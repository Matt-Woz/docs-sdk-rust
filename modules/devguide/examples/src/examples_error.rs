use std::fmt::Display;

#[derive(Debug)]
// This exists to allow the use of `?` with Couchbase SDK calls in examples.
pub struct ExamplesError {
    wrapped: couchbase::error::Error,
}

impl Display for ExamplesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.wrapped)
    }
}

impl std::error::Error for ExamplesError {}

impl From<couchbase::error::Error> for ExamplesError {
    fn from(value: couchbase::error::Error) -> Self {
        ExamplesError { wrapped: value }
    }
}
