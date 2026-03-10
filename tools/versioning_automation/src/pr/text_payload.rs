use crate::pr::commands::pr_text_payload_options::PrTextPayloadOptions;
use crate::pr::gh_cli::gh_output_trim_end_newline;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_text_payload(opts: PrTextPayloadOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    let title = gh_output_trim_end_newline(
        "pr",
        &[
            "view",
            &opts.pr_number,
            "-R",
            &repo_name,
            "--json",
            "title",
            "--jq",
            ".title // \"\"",
        ],
    )
    .unwrap_or_default();
    let body = gh_output_trim_end_newline(
        "pr",
        &[
            "view",
            &opts.pr_number,
            "-R",
            &repo_name,
            "--json",
            "body",
            "--jq",
            ".body // \"\"",
        ],
    )
    .unwrap_or_default();
    let commits = gh_output_trim_end_newline(
        "api",
        &[
            &format!("repos/{repo_name}/pulls/{}/commits", opts.pr_number),
            "--paginate",
            "--jq",
            ".[].commit.message",
        ],
    )
    .unwrap_or_default();

    print!("{title}\n{body}\n{commits}");
    0
}
