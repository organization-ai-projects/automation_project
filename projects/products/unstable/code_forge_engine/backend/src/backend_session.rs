use crate::contract::contract::Contract;
use crate::diagnostics::backend_error::BackendError;
use crate::generate::workspace_generator::WorkspaceGenerator;
use crate::io::fs_writer::FsWriter;
use crate::output::artifact_manifest::ArtifactManifest;
use crate::output::manifest_hash::ManifestHash;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::validate::contract_validator::ContractValidator;
use crate::validate::rule_validator::RuleValidator;
use std::path::Path;

#[derive(Default)]
pub struct BackendSession {
    contract_path: Option<String>,
    contract: Option<Contract>,
    manifest: Option<ArtifactManifest>,
    manifest_json: Option<String>,
    manifest_hash: Option<String>,
    shutdown_requested: bool,
}

impl BackendSession {
    pub fn should_shutdown(&self) -> bool {
        self.shutdown_requested
    }

    pub fn handle(&mut self, request: Request) -> Response {
        match self.handle_request(request) {
            Ok(response) => response,
            Err(error) => Response::error(3, "request_failed", &error.to_string()),
        }
    }

    fn handle_request(&mut self, request: Request) -> Result<Response, BackendError> {
        match request {
            Request::LoadContract { path } => {
                self.contract_path = Some(path.clone());
                self.contract = Some(load_contract_from_path(&path)?);
                Ok(Response::Ok)
            }
            Request::ValidateContract => {
                let contract = self
                    .contract
                    .as_ref()
                    .ok_or_else(|| BackendError::NotFound("contract not loaded".to_string()))?;
                ContractValidator::validate(contract)?;
                for rule in &contract.rules {
                    RuleValidator::validate(rule)?;
                }
                Ok(Response::Ok)
            }
            Request::PreviewLayout => {
                let contract = self
                    .contract
                    .as_ref()
                    .ok_or_else(|| BackendError::NotFound("contract not loaded".to_string()))?;
                let generated = WorkspaceGenerator::new(contract.clone()).generate()?;
                let mut files: Vec<String> = generated
                    .sorted_paths()
                    .iter()
                    .map(|s| (*s).to_string())
                    .collect();
                files.sort();
                Ok(Response::Preview { files })
            }
            Request::Generate { out_dir, mode } => {
                let contract = self
                    .contract
                    .as_ref()
                    .ok_or_else(|| BackendError::NotFound("contract not loaded".to_string()))?;
                let generated = WorkspaceGenerator::new(contract.clone()).generate()?;

                if mode == "write" {
                    for (path, bytes) in &generated.files {
                        let full_path = Path::new(&out_dir).join(path);
                        FsWriter::write(full_path, bytes)?;
                    }
                } else if mode != "dry_run" {
                    return Err(BackendError::Validation(format!(
                        "unknown generate mode: {mode}"
                    )));
                }

                let manifest_json = generated.canonical_json().map_err(BackendError::Encode)?;
                let manifest_hash = ManifestHash::compute(&manifest_json);

                self.manifest = Some(generated);
                self.manifest_json = Some(manifest_json);
                self.manifest_hash = Some(manifest_hash);
                Ok(Response::Ok)
            }
            Request::GetManifest => {
                let manifest_json = self
                    .manifest_json
                    .clone()
                    .ok_or_else(|| BackendError::NotFound("manifest not available".to_string()))?;
                let manifest_hash = self
                    .manifest_hash
                    .clone()
                    .ok_or_else(|| BackendError::NotFound("manifest hash missing".to_string()))?;
                Ok(Response::Manifest {
                    manifest_json,
                    manifest_hash,
                })
            }
            Request::Shutdown => {
                self.shutdown_requested = true;
                Ok(Response::Ok)
            }
        }
    }
}

fn load_contract_from_path(path: &str) -> Result<Contract, BackendError> {
    let raw = std::fs::read_to_string(path).map_err(|error| BackendError::Io(error.to_string()))?;

    if path.ends_with(".json") {
        common_json::from_str(&raw).map_err(|error| BackendError::Decode(error.to_string()))
    } else if path.ends_with(".toml") {
        toml::from_str(&raw).map_err(|error| BackendError::Decode(error.to_string()))
    } else {
        Err(BackendError::Validation(
            "contract file must be .json or .toml".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::BackendSession;
    use crate::protocol::request::Request;
    use std::io::Write;

    fn write_temp_contract(content: &str) -> String {
        let dir = std::env::temp_dir().join("code_forge_engine_tests");
        std::fs::create_dir_all(&dir).expect("create temp dir");
        let path = dir.join("contract.json");
        let mut file = std::fs::File::create(&path).expect("create contract file");
        writeln!(file, "{content}").expect("write contract");
        path.to_string_lossy().to_string()
    }

    #[test]
    fn generate_is_deterministic() {
        let contract_path = write_temp_contract(
            r#"{
  "name":"demo",
  "version":"0.1.0",
  "modules":[{
    "name":"core",
    "files":[{
      "path":"src/core/value.rs",
      "primary_type":"Value",
      "content_template":"pub struct {{primary_type}};"
    }]
  }],
  "rules":[{"id":"r1","description":"must be deterministic","enforced":true}]
}"#,
        );

        let mut session = BackendSession::default();
        assert!(matches!(
            session.handle(Request::LoadContract {
                path: contract_path.clone()
            }),
            crate::protocol::response::Response::Ok
        ));
        assert!(matches!(
            session.handle(Request::ValidateContract),
            crate::protocol::response::Response::Ok
        ));
        assert!(matches!(
            session.handle(Request::Generate {
                out_dir: "".to_string(),
                mode: "dry_run".to_string(),
            }),
            crate::protocol::response::Response::Ok
        ));

        let first_manifest = session.handle(Request::GetManifest);
        assert!(matches!(
            session.handle(Request::Generate {
                out_dir: "".to_string(),
                mode: "dry_run".to_string(),
            }),
            crate::protocol::response::Response::Ok
        ));
        let second_manifest = session.handle(Request::GetManifest);

        match (first_manifest, second_manifest) {
            (
                crate::protocol::response::Response::Manifest {
                    manifest_hash: hash_a,
                    manifest_json: json_a,
                },
                crate::protocol::response::Response::Manifest {
                    manifest_hash: hash_b,
                    manifest_json: json_b,
                },
            ) => {
                assert_eq!(hash_a, hash_b);
                assert_eq!(json_a, json_b);
            }
            _ => assert!(false, "expected manifest responses"),
        }
    }
}
