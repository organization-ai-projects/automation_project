use crate::components::Components;
use crate::diagnostics::error::Error;
use crate::screens::document_screen::DocumentScreen;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;
use crate::transport::request::Request;
use crate::transport::response::Response;
use dioxus::prelude::Element;
use dioxus::prelude::VNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub title: String,
    pub screen: DocumentScreen,
}

pub fn render() -> Element {
    let screen = DocumentScreen {
        heading: "Document".to_string(),
    };
    let app = App {
        title: "Docforge UI".to_string(),
        screen,
    };
    let backend = BackendProcess {
        binary: "docforge_backend".to_string(),
    };
    let client = IpcClient {
        channel: "stdio".to_string(),
    };
    let request = Request::LoadDocument {
        doc_id: "draft".to_string(),
    };
    let response = Response::DocumentLoaded {
        doc_id: "draft".to_string(),
    };
    let error = Error::Transport("none".to_string());
    let components = Components;
    let request_doc_id = match request {
        Request::LoadDocument { doc_id } => doc_id,
    };
    let response_doc_id = match response {
        Response::DocumentLoaded { doc_id } => doc_id,
    };
    let error_text = match error {
        Error::Transport(text) => text,
    };
    let components_text = format!("{components:?}");
    let summary = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        app.title,
        app.screen.heading,
        backend.binary,
        client.channel,
        request_doc_id,
        response_doc_id,
        error_text,
        components_text
    );
    let summary_len = summary.len();
    if summary_len == usize::MAX {
        return VNode::empty();
    }

    VNode::empty()
}
