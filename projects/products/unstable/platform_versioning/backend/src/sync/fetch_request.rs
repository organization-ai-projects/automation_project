// projects/products/unstable/platform_versioning/backend/src/sync/fetch_request.rs
use serde::{Deserialize, Serialize};

use crate::ids::ObjectId;
use crate::refs_store::RefName;

/// A request to fetch objects from the server.
///
/// The client provides a list of refs it wants and a list of objects it already
/// has. The server responds with all objects reachable from the requested refs
/// that are not in `have`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FetchRequest {
    /// The refs the client wants to fetch.
    pub want: Vec<RefName>,
    /// Object ids the client already has (to avoid re-sending).
    pub have: Vec<ObjectId>,
    /// Maximum number of objects to return in a single response (for pagination).
    pub limit: Option<usize>,
}
