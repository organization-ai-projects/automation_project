mod app;
mod diagnostics;
mod persistence;
mod plugins;
mod transport;
mod ui_model;

#[cfg(test)]
mod tests;

fn main() {
    let mut server = transport::ipc_server::IpcServer::new();
    let request_line = std::io::stdin().lines();
    for line in request_line {
        match line {
            Ok(input) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }
                match common_json::from_str::<transport::request::Request>(trimmed) {
                    Ok(request) => {
                        let response = server.handle(&request);
                        match common_json::to_string(&response) {
                            Ok(json) => println!("{json}"),
                            Err(e) => {
                                eprintln!("Serialization error: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        let err_response = transport::response::Response::Error {
                            message: format!("Invalid request: {e}"),
                        };
                        if let Ok(json) = common_json::to_string(&err_response) {
                            println!("{json}");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("IO error: {e}");
                break;
            }
        }
    }
}
