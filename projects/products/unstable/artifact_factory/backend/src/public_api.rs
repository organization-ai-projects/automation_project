use crate::analyze::dependency_graph::DependencyGraph;
use crate::analyze::event_map::EventMap;
use crate::analyze::protocol_map::ProtocolMap;
use crate::bundle::artifact_bundle::ArtifactBundle;
use crate::bundle::bundle_hash::BundleHash;
use crate::diagnostics::backend_error::BackendError;
use crate::input::artifact_input::ArtifactInput;
use crate::input::artifact_kind::ArtifactKind;
use crate::input::input_loader::InputLoader;
use crate::io::fs_writer::FsWriter;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::render::html_renderer::HtmlRenderer;
use crate::render::markdown_renderer::MarkdownRenderer;
use crate::render::svg_renderer::SvgRenderer;

/// Session state shared across requests in a single backend run.
#[derive(Default)]
pub struct BackendSession {
    pub inputs: Vec<ArtifactInput>,
    pub event_map: Option<EventMap>,
    pub protocol_map: Option<ProtocolMap>,
    pub dep_graph: Option<DependencyGraph>,
    pub bundle: Option<ArtifactBundle>,
    pub bundle_hash: Option<String>,
}

impl BackendSession {
    pub fn handle(&mut self, request: Request) -> Result<Response, BackendError> {
        match request {
            Request::LoadInputs { paths } => {
                self.inputs = InputLoader::load_from_paths(&paths)?;
                tracing::info!(count = self.inputs.len(), "inputs loaded");
                let (reports, replays, manifests, protocol_schemas, unknown) =
                    summarize_input_kinds(&self.inputs);
                Ok(Response::InputsLoaded {
                    total: self.inputs.len(),
                    reports,
                    replays,
                    manifests,
                    protocol_schemas,
                    unknown,
                })
            }
            Request::Analyze => {
                if self.inputs.is_empty() {
                    return Err(BackendError::Analysis(
                        "no inputs loaded; run LoadInputs first".to_string(),
                    ));
                }
                let event_map = EventMap::build(&self.inputs);
                let protocol_map = ProtocolMap::build(&self.inputs);
                let dep_graph = DependencyGraph::build(&self.inputs);
                tracing::info!(
                    events = event_map.len(),
                    protocols = protocol_map.len(),
                    nodes = dep_graph.node_count(),
                    "analysis complete"
                );
                self.event_map = Some(event_map);
                self.protocol_map = Some(protocol_map);
                self.dep_graph = Some(dep_graph);
                let edge_count = self
                    .dep_graph
                    .as_ref()
                    .map(|graph| graph.edges.values().map(std::vec::Vec::len).sum())
                    .unwrap_or(0usize);
                Ok(Response::AnalysisComplete {
                    events: self.event_map.as_ref().map(EventMap::len).unwrap_or(0),
                    protocols: self
                        .protocol_map
                        .as_ref()
                        .map(ProtocolMap::len)
                        .unwrap_or(0),
                    nodes: self
                        .dep_graph
                        .as_ref()
                        .map(DependencyGraph::node_count)
                        .unwrap_or(0),
                    edges: edge_count,
                })
            }
            Request::RenderDocs => {
                if self.inputs.is_empty() {
                    return Err(BackendError::Render(
                        "no inputs loaded; run LoadInputs first".to_string(),
                    ));
                }
                let event_map = self.event_map.as_ref().cloned().unwrap_or_default();
                let protocol_map = self.protocol_map.as_ref().cloned().unwrap_or_default();
                let dep_graph = self.dep_graph.as_ref().cloned().unwrap_or_default();
                let md =
                    MarkdownRenderer::render(&self.inputs, &event_map, &protocol_map, &dep_graph);
                let svg = SvgRenderer::render(&dep_graph);
                let html =
                    HtmlRenderer::render(&self.inputs, &event_map, &protocol_map, &dep_graph);
                let markdown_bytes = md.len();
                let svg_bytes = svg.len();
                let html_bytes = html.len();
                let mut bundle = ArtifactBundle::new();
                bundle.add_file("docs.md", md.into_bytes());
                bundle.add_file("graph.svg", svg.into_bytes());
                bundle.add_file("docs.html", html.into_bytes());
                let hash = BundleHash::compute(&bundle);
                tracing::info!(hash = %hash, "docs rendered");
                self.bundle_hash = Some(hash);
                self.bundle = Some(bundle);
                Ok(Response::DocsRendered {
                    markdown_bytes,
                    svg_bytes,
                    html_bytes,
                })
            }
            Request::BuildBundle => {
                let bundle = self.bundle.get_or_insert_with(ArtifactBundle::new);
                let hash = BundleHash::compute(bundle);
                self.bundle_hash = Some(hash);
                if let Ok(output_dir) = std::env::var("ARTIFACT_FACTORY_OUTPUT_DIR") {
                    FsWriter::write_bundle(bundle, std::path::Path::new(&output_dir))
                        .map_err(|err| BackendError::Bundle(err.to_string()))?;
                }
                Ok(Response::Ok)
            }
            Request::GetBundle => match (&self.bundle, &self.bundle_hash) {
                (Some(bundle), Some(hash)) => Ok(Response::Bundle {
                    hash: hash.clone(),
                    manifest: bundle.file_names().to_vec(),
                }),
                _ => Ok(Response::Error {
                    message: "no bundle available; run RenderDocs or BuildBundle first".to_string(),
                }),
            },
        }
    }
}

fn summarize_input_kinds(inputs: &[ArtifactInput]) -> (usize, usize, usize, usize, usize) {
    let mut reports = 0usize;
    let mut replays = 0usize;
    let mut manifests = 0usize;
    let mut protocol_schemas = 0usize;
    let mut unknown = 0usize;

    for input in inputs {
        match &input.kind {
            ArtifactKind::Report => reports += 1,
            ArtifactKind::Replay => replays += 1,
            ArtifactKind::Manifest => manifests += 1,
            ArtifactKind::ProtocolSchema => protocol_schemas += 1,
            ArtifactKind::Unknown => unknown += 1,
        }
    }

    (reports, replays, manifests, protocol_schemas, unknown)
}
