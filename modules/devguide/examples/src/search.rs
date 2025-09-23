use crate::examples_error::ExamplesError;
use couchbase::mutation_state::MutationState;
// #tag::imports[]
use couchbase::options::search_options::SearchOptions;
use couchbase::scope::Scope;
use couchbase::search::queries::{MatchAllQuery, MatchQuery, Query};
use couchbase::search::request::SearchRequest;
use couchbase::search::vector::{VectorQuery, VectorSearch};
use futures::StreamExt;
use serde_json::json;
// #end::imports[]

pub async fn vector_search(scope: Scope) -> Result<(), ExamplesError> {
    let vector_query = vec![1.0, 2.0, 3.0, 4.0]; // Example vector data
    // #tag::vectorsearch[]
    let request = SearchRequest::with_vector_search(VectorSearch::new(
        vec![VectorQuery::with_vector("vector_field", vector_query)],
        None,
    ));

    let result = scope.search("vector-index", request, None).await?;
    // #end::vectorsearch[]

    Ok(())
}

pub async fn basic_search(scope: Scope) -> Result<(), ExamplesError> {
    // #tag::basicsearch[]
    let result = scope
        .search(
            "travel-sample-index-hotel-description",
            SearchRequest::with_search_query(Query::Match(MatchQuery::new("swanky"))),
            SearchOptions::new().limit(10),
        )
        .await;

    match result {
        Ok(mut res) => {
            println!("Search successful");
            let rows = res.rows();
            // use rows
        }
        Err(e) => {
            println!("Error performing search: {e}");
        }
    }
    // #end::basicsearch[]

    Ok(())
}

pub async fn handling_results(scope: Scope) -> Result<(), ExamplesError> {
    // #tag::handlingresults[]
    let mut result = scope
        .search(
            "travel-sample-index-hotel-description",
            SearchRequest::with_search_query(Query::Match(MatchQuery::new("swanky"))),
            SearchOptions::new().limit(10),
        )
        .await?;

    {
        let mut rows = result.rows();
        while let Some(row) = rows.next().await {
            // Each individual row read can error.
            let row = row?;
            let id = row.id;
            let score = row.score;
            // ...
        }
    }

    // Metadata - can only be read after all rows have been read.
    let metadata = result.metadata()?;
    let total_hits = metadata.metrics.total_hits;
    let success_count = metadata.metrics.max_score;
    // #end::handlingresults[]

    Ok(())
}

pub async fn consistency(scope: Scope) -> Result<(), ExamplesError> {
    // #tag::consistency[]
    let insert_result = scope
        .collection("hotels")
        .insert(
            "newHotel",
            json!({"name": "Hotel California", "desc": "Such a lonely place"}),
            None,
        )
        .await?;

    let mutation_state =
        MutationState::new_with_tokens(vec![insert_result.mutation_token().clone().unwrap()]);

    let result = scope
        .search(
            "travel-sample-index-hotel-description",
            SearchRequest::with_search_query(Query::Match(MatchQuery::new("lonely"))),
            SearchOptions::new()
                .limit(10)
                .consistent_with(mutation_state),
        )
        .await?;
    // #end::consistency[]

    Ok(())
}

pub async fn multiple_vectors(scope: Scope) -> Result<(), ExamplesError> {
    let vector_query1 = vec![1.0, 2.0, 3.0, 4.0]; // Example vector data
    let vector_query2 = vec![4.0, 3.0, 2.0, 1.0]; // Example vector data
    // #tag::multiplevectors[]
    let request = SearchRequest::with_vector_search(VectorSearch::new(
        vec![
            VectorQuery::with_vector("vector_field", vector_query1)
                .num_candidates(2)
                .boost(0.3),
            VectorQuery::with_vector("vector_field", vector_query2)
                .num_candidates(5)
                .boost(0.7),
        ],
        None,
    ));

    let result = scope.search("vector-index", request, None).await?;
    // #end::multiplevectors[]

    Ok(())
}

pub async fn combine_search_vector(scope: Scope) -> Result<(), ExamplesError> {
    let vector_query = vec![1.0, 2.0, 3.0, 4.0]; // Example vector data
    // #tag::combinesearchvector[]
    let vector_search = VectorSearch::new(
        vec![VectorQuery::with_vector("vector_field", vector_query)],
        None,
    );
    let fts_search = Query::MatchAll(MatchAllQuery::new());

    let request = SearchRequest::with_search_query(fts_search).vector_search(vector_search)?;

    let result = scope.search("vector-index", request, None).await?;
    // #end::combinesearchvector[]

    Ok(())
}
