use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

#[test]
#[ignore = "requires local TCP socket binding"]
fn serve_metrics_admin_profile_rejects_missing_token() {
    let addr = pick_unused_local_addr();
    let args = vec![
        addr.clone(),
        "--once".to_string(),
        "--admin-token".to_string(),
        "secret-token".to_string(),
        "--disable-auto-rollback".to_string(),
    ];

    let server =
        thread::spawn(move || crate::app::cmd_serve_metrics(&args).map_err(|e| e.to_string()));
    let response = send_http_request(
        &addr,
        "POST /admin/slo-profile?profile=strict HTTP/1.1\r\nHost: local\r\nConnection: close\r\n\r\n",
    );
    let server_result = server.join().expect("server thread should not panic");
    assert!(server_result.is_ok());
    assert!(response.contains("401 Unauthorized"));
}

#[test]
#[ignore = "requires local TCP socket binding"]
fn serve_metrics_admin_audit_endpoint_returns_applied_switch() {
    let audit_path = unique_test_file_path("admin_audit_http");
    let audit_path_text = audit_path.to_str().expect("utf8 path").to_string();

    let post_addr = pick_unused_local_addr();
    let post_args = vec![
        post_addr.clone(),
        "--once".to_string(),
        "--admin-token".to_string(),
        "secret-token".to_string(),
        "--slo-audit-path".to_string(),
        audit_path_text.clone(),
        "--disable-auto-rollback".to_string(),
    ];
    let post_server =
        thread::spawn(move || crate::app::cmd_serve_metrics(&post_args).map_err(|e| e.to_string()));
    let post_response = send_http_request(
        &post_addr,
        "POST /admin/slo-profile?profile=strict HTTP/1.1\r\nHost: local\r\nX-Admin-Token: secret-token\r\nConnection: close\r\n\r\n",
    );
    let post_result = post_server.join().expect("server thread should not panic");
    assert!(post_result.is_ok());
    assert!(post_response.contains("200 OK"));

    let get_addr = pick_unused_local_addr();
    let get_args = vec![
        get_addr.clone(),
        "--once".to_string(),
        "--admin-token".to_string(),
        "secret-token".to_string(),
        "--slo-audit-path".to_string(),
        audit_path_text.clone(),
    ];
    let get_server =
        thread::spawn(move || crate::app::cmd_serve_metrics(&get_args).map_err(|e| e.to_string()));
    let get_response = send_http_request(
        &get_addr,
        "GET /admin/slo-audit?limit=10 HTTP/1.1\r\nHost: local\r\nX-Admin-Token: secret-token\r\nConnection: close\r\n\r\n",
    );
    let get_result = get_server.join().expect("server thread should not panic");
    assert!(get_result.is_ok());
    assert!(get_response.contains("200 OK"));
    assert!(get_response.contains("\"result\":\"applied\""));

    let _ = std::fs::remove_file(&audit_path);
    let _ = std::fs::remove_file(format!("{}.1", audit_path.display()));
}

fn pick_unused_local_addr() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let addr = listener.local_addr().expect("resolve local addr");
    format!("127.0.0.1:{}", addr.port())
}

fn send_http_request(addr: &str, request: &str) -> String {
    let mut stream = match connect_with_retry(addr, 40, Duration::from_millis(10)) {
        Ok(stream) => stream,
        Err(err) => return format!("CONNECT_ERROR: {err}"),
    };

    stream
        .write_all(request.as_bytes())
        .expect("send HTTP request");
    stream
        .shutdown(std::net::Shutdown::Write)
        .expect("shutdown write");
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("read HTTP response");
    response
}

fn connect_with_retry(addr: &str, max_attempts: u32, delay: Duration) -> Result<TcpStream, String> {
    let mut attempts = 0_u32;
    loop {
        match TcpStream::connect(addr) {
            Ok(stream) => return Ok(stream),
            Err(_) if attempts < max_attempts => {
                attempts += 1;
                thread::sleep(delay);
            }
            Err(err) => return Err(err.to_string()),
        }
    }
}

fn unique_test_file_path(prefix: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let suffix = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "neurosymbolic_moe_{prefix}_{}_{}.jsonl",
        std::process::id(),
        suffix
    ))
}
