use crate::diagnostics::Error;
use crate::query::Query;
use crate::query_result::QueryResult;
use crate::snapshot::Snapshot;

pub struct QueryEngine;

impl QueryEngine {
    pub fn execute(snapshot: &Snapshot, query: &Query) -> Result<QueryResult, Error> {
        let mut matches: Vec<String> = match query {
            Query::ReverseDeps { crate_name } => {
                let rev = snapshot.crate_graph.reverse_deps(crate_name);
                rev.into_iter().map(|c| c.name.clone()).collect()
            }
            Query::PublicItems { crate_name } => snapshot
                .public_items
                .iter()
                .filter(|item| item.crate_name == *crate_name)
                .map(|item| item.name.clone())
                .collect(),
            Query::FindSymbol { substring } => snapshot
                .public_items
                .iter()
                .filter(|item| item.name.contains(substring.as_str()))
                .map(|item| {
                    format!("{}::{}::{}", item.crate_name, item.module_path, item.name)
                })
                .collect(),
        };

        matches.sort();
        matches.dedup();

        Ok(QueryResult {
            query: query.clone(),
            matches,
        })
    }
}
