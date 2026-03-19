use std::{env, path, process};

use tokio::{fs, task, time};

use crate::{runtime::flush_and_stop_periodic_task, store::AccountManager};

fn temp_data_dir() -> path::PathBuf {
    let unique = format!(
        "accounts_runtime_test_{}_{}",
        process::id(),
        protocol::Metadata::current_timestamp_ms()
    );
    env::temp_dir().join(unique)
}

#[tokio::test]
async fn flush_and_stop_periodic_task_aborts_task_and_flushes_cleanly() {
    let data_dir = temp_data_dir();
    let manager = AccountManager::load(data_dir.clone())
        .await
        .expect("manager should load");

    let handle = task::spawn(async {
        time::sleep(time::Duration::from_secs(60)).await;
    });

    flush_and_stop_periodic_task(&handle, &manager, "during runtime test").await;

    let mut attempts = 0u8;
    while !handle.is_finished() && attempts < 10 {
        task::yield_now().await;
        time::sleep(time::Duration::from_millis(5)).await;
        attempts += 1;
    }
    assert!(handle.is_finished());

    let removed = fs::remove_dir_all(data_dir).await;
    assert!(removed.is_ok());
}
