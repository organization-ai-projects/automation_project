//projects/products/core/central_ui/src/filters.rs
use warp::Filter;

pub(crate) fn with_client(
    client: reqwest::Client,
) -> impl Filter<Extract = (reqwest::Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub(crate) fn with_engine_base(
    engine_base: String,
) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || engine_base.clone())
}

pub(crate) fn with_claim_dir(
    claim_dir: Option<String>,
) -> impl Filter<Extract = (Option<String>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || claim_dir.clone())
}
