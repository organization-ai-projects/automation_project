use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_path(name: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!("docforge_{name}_{}_{}.json", std::process::id(), n))
}

#[test]
fn test_cli_create_edit_render_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let bin = env!("CARGO_BIN_EXE_docforge_backend");
    let doc_file = unique_path("doc");
    let ops_file = unique_path("ops");

    let status_new = Command::new(bin)
        .arg("new")
        .arg("--title")
        .arg("Integration")
        .arg("--out")
        .arg(&doc_file)
        .status()?;
    assert!(status_new.success());

    let ops = r#"[
  {
    "InsertBlock": {
      "position": 0,
      "block": {
        "Paragraph": {
          "id": "p1",
          "content": [{"Text": "Hello"}],
          "style": null
        }
      }
    }
  },
  {
    "SetTitle": {
      "title": "Updated"
    }
  }
]"#;
    std::fs::write(&ops_file, ops)?;

    let status_edit = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--apply")
        .arg(&ops_file)
        .status()?;
    assert!(status_edit.success());
    let saved = std::fs::read_to_string(&doc_file)?;
    assert!(saved.contains("\"events\""));
    assert!(saved.contains("\"sequence\": 1"));

    let output_html = Command::new(bin)
        .arg("render")
        .arg(&doc_file)
        .arg("--target")
        .arg("html")
        .output()?;
    assert!(output_html.status.success());
    let html = String::from_utf8(output_html.stdout)?;
    assert!(html.contains("<p>Hello</p>"));

    let output_text = Command::new(bin)
        .arg("render")
        .arg(&doc_file)
        .arg("--target")
        .arg("text")
        .output()?;
    assert!(output_text.status.success());
    let text = String::from_utf8(output_text.stdout)?;
    assert!(text.contains("Updated"));
    assert!(text.contains("Hello"));

    let replay_text = Command::new(bin)
        .arg("replay")
        .arg(&doc_file)
        .arg("--target")
        .arg("text")
        .output()?;
    assert!(replay_text.status.success());
    let replayed = String::from_utf8(replay_text.stdout)?;
    assert_eq!(text, replayed);

    Ok(())
}

#[test]
fn test_replay_deterministic_output() -> Result<(), Box<dyn std::error::Error>> {
    let bin = env!("CARGO_BIN_EXE_docforge_backend");
    let doc_file = unique_path("doc_replay");
    let ops_file = unique_path("ops_replay");

    let status_new = Command::new(bin)
        .arg("new")
        .arg("--title")
        .arg("Replay")
        .arg("--out")
        .arg(&doc_file)
        .status()?;
    assert!(status_new.success());

    let ops = r#"[
  {
    "InsertBlock": {
      "position": 0,
      "block": {
        "Heading": {
          "id": "h1",
          "level": 1,
          "content": [{"Text": "Title"}],
          "style": null
        }
      }
    }
  }
]"#;
    std::fs::write(&ops_file, ops)?;

    let status_edit = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--apply")
        .arg(&ops_file)
        .status()?;
    assert!(status_edit.success());
    let saved = std::fs::read_to_string(&doc_file)?;
    assert!(saved.contains("\"events\""));
    assert!(saved.contains("\"sequence\": 1"));

    let first = Command::new(bin)
        .arg("render")
        .arg(&doc_file)
        .arg("--target")
        .arg("html")
        .output()?;
    let second = Command::new(bin)
        .arg("render")
        .arg(&doc_file)
        .arg("--target")
        .arg("html")
        .output()?;

    assert!(first.status.success());
    assert!(second.status.success());
    assert_eq!(first.stdout, second.stdout);

    Ok(())
}

#[test]
fn test_cli_edit_undo_reverts_last_event() -> Result<(), Box<dyn std::error::Error>> {
    let bin = env!("CARGO_BIN_EXE_docforge_backend");
    let doc_file = unique_path("doc_undo");
    let ops_file = unique_path("ops_undo");

    let status_new = Command::new(bin)
        .arg("new")
        .arg("--title")
        .arg("Undo")
        .arg("--out")
        .arg(&doc_file)
        .status()?;
    assert!(status_new.success());

    let ops = r#"[
  {
    "InsertBlock": {
      "position": 0,
      "block": {
        "Paragraph": {
          "id": "p1",
          "content": [{"Text": "Hello"}],
          "style": null
        }
      }
    }
  }
]"#;
    std::fs::write(&ops_file, ops)?;

    let status_edit = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--apply")
        .arg(&ops_file)
        .status()?;
    assert!(status_edit.success());

    let before_undo = Command::new(bin)
        .arg("render")
        .arg(&doc_file)
        .arg("--target")
        .arg("text")
        .output()?;
    assert!(before_undo.status.success());
    let before = String::from_utf8(before_undo.stdout)?;
    assert!(before.contains("Hello"));

    let status_undo = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--undo")
        .status()?;
    assert!(status_undo.success());

    let after_undo = Command::new(bin)
        .arg("render")
        .arg(&doc_file)
        .arg("--target")
        .arg("text")
        .output()?;
    assert!(after_undo.status.success());
    let after = String::from_utf8(after_undo.stdout)?;
    assert!(!after.contains("Hello"));

    Ok(())
}

#[test]
fn test_cli_edit_redo_restores_last_undone_event() -> Result<(), Box<dyn std::error::Error>> {
    let bin = env!("CARGO_BIN_EXE_docforge_backend");
    let doc_file = unique_path("doc_redo");
    let ops_file = unique_path("ops_redo");

    let status_new = Command::new(bin)
        .arg("new")
        .arg("--title")
        .arg("Redo")
        .arg("--out")
        .arg(&doc_file)
        .status()?;
    assert!(status_new.success());

    let ops = r#"[
  {
    "InsertBlock": {
      "position": 0,
      "block": {
        "Paragraph": {
          "id": "p1",
          "content": [{"Text": "Hello"}],
          "style": null
        }
      }
    }
  }
]"#;
    std::fs::write(&ops_file, ops)?;

    let status_edit = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--apply")
        .arg(&ops_file)
        .status()?;
    assert!(status_edit.success());

    let status_undo = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--undo")
        .status()?;
    assert!(status_undo.success());

    let status_redo = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--redo")
        .status()?;
    assert!(status_redo.success());

    let rendered = Command::new(bin)
        .arg("render")
        .arg(&doc_file)
        .arg("--target")
        .arg("text")
        .output()?;
    assert!(rendered.status.success());
    let text = String::from_utf8(rendered.stdout)?;
    assert!(text.contains("Hello"));

    Ok(())
}

#[test]
fn test_cli_history_reports_event_counters() -> Result<(), Box<dyn std::error::Error>> {
    let bin = env!("CARGO_BIN_EXE_docforge_backend");
    let doc_file = unique_path("doc_history");
    let ops_file = unique_path("ops_history");

    let status_new = Command::new(bin)
        .arg("new")
        .arg("--title")
        .arg("History")
        .arg("--out")
        .arg(&doc_file)
        .status()?;
    assert!(status_new.success());

    let ops = r#"[
  {
    "SetTitle": {
      "title": "History Updated"
    }
  }
]"#;
    std::fs::write(&ops_file, ops)?;

    let status_edit = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--apply")
        .arg(&ops_file)
        .status()?;
    assert!(status_edit.success());

    let status_undo = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--undo")
        .status()?;
    assert!(status_undo.success());

    let status_redo = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--redo")
        .status()?;
    assert!(status_redo.success());

    let history = Command::new(bin).arg("history").arg(&doc_file).output()?;
    assert!(history.status.success());
    let text = String::from_utf8(history.stdout)?;
    assert!(text.contains("events=2"));
    assert!(text.contains("undone=0"));
    assert!(text.contains("last_sequence=2"));

    Ok(())
}

#[test]
fn test_cli_history_json_and_doc_id_filter() -> Result<(), Box<dyn std::error::Error>> {
    let bin = env!("CARGO_BIN_EXE_docforge_backend");
    let doc_file = unique_path("doc_history_json");

    let status_new = Command::new(bin)
        .arg("new")
        .arg("--title")
        .arg("History Json")
        .arg("--out")
        .arg(&doc_file)
        .status()?;
    assert!(status_new.success());

    let text_history = Command::new(bin).arg("history").arg(&doc_file).output()?;
    assert!(text_history.status.success());
    let text = String::from_utf8(text_history.stdout)?;
    let prefix = "doc_id=";
    let start = text.find(prefix).ok_or("missing doc_id prefix")? + prefix.len();
    let end = text[start..]
        .find(' ')
        .ok_or("missing doc_id suffix")?
        .checked_add(start)
        .ok_or("invalid doc_id span")?;
    let doc_id = &text[start..end];

    let json_history = Command::new(bin)
        .arg("history")
        .arg(&doc_file)
        .arg("--json")
        .arg("--doc-id")
        .arg(doc_id)
        .output()?;
    assert!(json_history.status.success());
    let json = String::from_utf8(json_history.stdout)?;
    assert!(json.contains("\"doc_id\""));
    assert!(json.contains(doc_id));
    assert!(json.contains("\"events\":1"));

    let filtered_empty = Command::new(bin)
        .arg("history")
        .arg(&doc_file)
        .arg("--json")
        .arg("--doc-id")
        .arg("missing-doc-id")
        .output()?;
    assert!(filtered_empty.status.success());
    let empty = String::from_utf8(filtered_empty.stdout)?;
    assert_eq!(empty.trim(), "[]");

    Ok(())
}

#[test]
fn test_cli_layout_outputs_layout_json() -> Result<(), Box<dyn std::error::Error>> {
    let bin = env!("CARGO_BIN_EXE_docforge_backend");
    let doc_file = unique_path("doc_layout");
    let ops_file = unique_path("ops_layout");

    let status_new = Command::new(bin)
        .arg("new")
        .arg("--title")
        .arg("Layout")
        .arg("--out")
        .arg(&doc_file)
        .status()?;
    assert!(status_new.success());

    let ops = r#"[
  {
    "InsertBlock": {
      "position": 0,
      "block": {
        "Paragraph": {
          "id": "p1",
          "content": [{"Text": "L"}],
          "style": null
        }
      }
    }
  }
]"#;
    std::fs::write(&ops_file, ops)?;
    let status_edit = Command::new(bin)
        .arg("edit")
        .arg(&doc_file)
        .arg("--apply")
        .arg(&ops_file)
        .status()?;
    assert!(status_edit.success());

    let layout = Command::new(bin).arg("layout").arg(&doc_file).output()?;
    assert!(layout.status.success());
    let json = String::from_utf8(layout.stdout)?;
    assert!(json.contains("\"kind\":\"paragraph\""));

    Ok(())
}
