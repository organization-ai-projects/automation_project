// projects/products/core/launcher/src/supervisor/logger.rs
use std::{
    io::{BufRead, BufReader},
    thread,
};

use crate::{child_handle::ChildHandle, logging::log_message};

use super::{locks::lock_recover, log_stream::LogStream};

fn spawn_logger<R: std::io::Read + Send + 'static>(
    stream: Option<R>,
    name: String,
    log_stream: LogStream,
) {
    if let Some(stream) = stream {
        thread::spawn(move || {
            let reader = BufReader::new(stream);
            for line in reader.lines() {
                match line {
                    Ok(line) => match log_stream {
                        LogStream::Stdout => log_message(&line, false, &name),
                        LogStream::Stderr => log_message(&line, true, &name),
                    },
                    Err(e) => {
                        log_message(&format!("log stream error: {}", e), true, &name);
                        break;
                    }
                }
            }
        });
    }
}

pub(crate) fn pipe_child_outputs(name: String, handle: ChildHandle) {
    let (stdout, stderr) = {
        let mut child = lock_recover(&handle.child, "child");
        (child.stdout.take(), child.stderr.take())
    };

    spawn_logger(stdout, name.clone(), LogStream::Stdout);
    spawn_logger(stderr, name, LogStream::Stderr);
}
