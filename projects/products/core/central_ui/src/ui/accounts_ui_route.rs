//projects/products/core/central_ui/src/ui/accounts_ui_route.rs
use std::path::PathBuf;

use warp::{Filter, Reply, filters::fs, path, reject::Rejection};

use crate::ui::validate_bundle::validate_bundle;

pub(crate) fn accounts_ui_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
{
    let ui_dist = std::env::var("CENTRAL_UI_ACCOUNTS_UI_DIST")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("projects/products/accounts/ui/ui_dist"));
    let ui_public = ui_dist.join("public");

    if let Err(missing) = validate_bundle(&ui_dist) {
        for item in missing {
            eprintln!("central_ui: missing accounts UI bundle file: {}", item);
        }
    }

    let index = path::end()
        .and(fs::file(ui_public.join("index.html")))
        .map(|file: fs::File| file.into_response());

    let files = fs::dir(ui_public);

    index.or(files).boxed()
}
