//! tools/versioning_automation/src/issues/commands/subissue_refs_options.rs
use crate::{gh_cli::output_trim_or_empty, issues::execute::print_non_empty_lines};

#[derive(Debug, Clone)]
pub(crate) struct SubissueRefsOptions {
    pub(crate) owner: String,
    pub(crate) repo: String,
    pub(crate) issue: String,
}

impl SubissueRefsOptions {
    pub(crate) fn run_subissue_refs(self) -> i32 {
        let query = "query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){issue(number:$number){subIssues(first:100){nodes{number}}}}}";
        let number_as_int = self.issue.parse::<u32>().unwrap_or_default().to_string();
        let output = output_trim_or_empty(&[
            "api",
            "graphql",
            "-f",
            &format!("query={query}"),
            "-f",
            &format!("owner={}", self.owner),
            "-f",
            &format!("name={}", self.repo),
            "-F",
            &format!("number={number_as_int}"),
            "--jq",
            ".data.repository.issue.subIssues.nodes[]?.number | \"#\"+tostring",
        ]);
        print_non_empty_lines(&output);
        0
    }
}
