use crate::pr::commands::pr_field_name::PrFieldName;
use crate::pr::commands::pr_field_options::PrFieldOptions;
use crate::pr_remote_snapshot::load_pr_remote_snapshot;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_field(opts: PrFieldOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    match opts.name {
        PrFieldName::CommitMessages => {
            let out = load_pr_remote_snapshot(&opts.pr_number, &repo_name)
                .map(|snapshot| snapshot.commit_messages)
                .unwrap_or_default();
            print!("{out}");
            0
        }
        _ => {
            let snapshot = load_pr_remote_snapshot(&opts.pr_number, &repo_name).unwrap_or_default();
            let value = match opts.name {
                PrFieldName::State => snapshot.state,
                PrFieldName::BaseRefName => snapshot.base_ref_name,
                PrFieldName::HeadRefName => snapshot.head_ref_name,
                PrFieldName::Title => snapshot.title,
                PrFieldName::Body => snapshot.body,
                PrFieldName::AuthorLogin => snapshot.author_login,
                PrFieldName::CommitMessages => String::new(),
            };
            println!("{value}");
            0
        }
    }
}
