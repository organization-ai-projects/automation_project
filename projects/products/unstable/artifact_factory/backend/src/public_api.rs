use crate::analyze::dependency_graph::DependencyGraph;
use crate::analyze::event_map::EventMap;
use crate::analyze::protocol_map::ProtocolMap;
use crate::bundle::artifact_bundle::ArtifactBundle;
use crate::bundle::bundle_hash::BundleHash;
use crate::diagnostics::error::FactoryError;
use crate::input::artifact_input::ArtifactInput;
use crate::input::input_loader::InputLoader;
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
    pub fn handle(&mut self, request: Request) -> Result<Response, FactoryError> {
        match request {
            Request::LoadInputs { paths } => {
                self.inputs = InputLoader::load_from_paths(&paths)?;
                tracing::info!(count = self.inputs.len(), "inputs loaded");
                Ok(Response::Ok)
            }
            Request::Analyze => {
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
                Ok(Response::Ok)
            }
            Request::RenderDocs => {
                let event_map = self.event_map.as_ref().cloned().unwrap_or_default();
                let protocol_map = self.protocol_map.as_ref().cloned().unwrap_or_default();
                let dep_graph = self.dep_graph.as_ref().cloned().unwrap_or_default();
                let md = MarkdownRenderer::render(&self.inputs, &event_map, &protocol_map, &dep_graph);
                let svg = SvgRenderer::render(&dep_graph);
                let html = HtmlRenderer::render(&self.inputs, &event_map, &protocol_map, &dep_graph);
                let mut bundle = ArtifactBundle::new();
                bundle.add_file("docs.md", md.into_bytes());
                bundle.add_file("graph.svg", svg.into_bytes());
                bundle.add_file("docs.html", html.into_bytes());
                let hash = BundleHash::compute(&bundle);
                tracing::info!(hash = %hash, "docs rendered");
                self.bundle_hash = Some(hash);
                self.bundle = Some(bundle);
                Ok(Response::Ok)
            }
            Request::BuildBundle => {
                let bundle = self.bundle.get_or_insert_with(ArtifactBundle::new);
                let hash = BundleHash::compute(bundle);
                self.bundle_hash = Some(hash);
                Ok(Response::Ok)
            }
            Request::GetBundle => {
                match (&self.bundle, &self.bundle_hash) {
                    (Some(bundle), Some(hash)) => Ok(Response::Bundle {
                        hash: hash.clone(),
                        manifest: bundle.manifest.clone(),
                    }),
                    _ => Ok(Response::Error {
                        message: "no bundle available; run RenderDocs or BuildBundle first".to_string(),
                    }),
                }
            }
        }
    }
}
