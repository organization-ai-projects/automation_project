use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum RestartPolicy {
    Always,
    #[default]
    OnFailure,
    Never,
}
