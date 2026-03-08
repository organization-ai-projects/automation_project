#[cfg(not(target_arch = "wasm32"))]
use crate::transport::client_port::ClientPort;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::request::Request;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::response::Response;

#[cfg(not(target_arch = "wasm32"))]
struct NoopClient;

#[cfg(not(target_arch = "wasm32"))]
impl ClientPort for NoopClient {
    fn send(&self, _request: Request) -> Response {
        Response::Ok
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn client_port_trait_object_is_callable() {
    let client: Box<dyn ClientPort> = Box::new(NoopClient);
    let response = client.send(Request::Health);
    assert_eq!(response, Response::Ok);
}

#[cfg(target_arch = "wasm32")]
#[test]
fn client_port_related_types_are_available_on_wasm() {
    let response = crate::transport::response::Response::Ok;
    assert_eq!(response, crate::transport::response::Response::Ok);
}
