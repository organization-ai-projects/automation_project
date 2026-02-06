use std::fs;
use auto_manager_ai::{Config, generate_action_plan, write_outputs, RunReport};

#[test]
fn test_no_repo_writes() {
    // Create a temporary directory for testing
    let temp_dir = std::env::temp_dir().join(format!("auto_manager_ai_test_{}", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()));
    
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    
    // Create a test file
    let test_file = temp_dir.join("test.txt");
    fs::write(&test_file, "original content").expect("Failed to write test file");
    
    // Create output directory
    let out_dir = temp_dir.join("out");
    
    // Create config pointing to temp directory
    let config = Config::new(temp_dir.clone(), out_dir.clone());
    
    // Generate action plan
    let plan = generate_action_plan(&config).expect("Failed to generate plan");
    
    // Verify test file is unchanged
    let content = fs::read_to_string(&test_file).expect("Failed to read test file");
    assert_eq!(content, "original content", "Repository file was modified!");
    
    // Write outputs (this is allowed)
    let report = RunReport::new("test_run".to_string());
    write_outputs(&plan, &report, &out_dir).expect("Failed to write outputs");
    
    // Verify outputs were written to out directory only
    assert!(out_dir.join("action_plan.json").exists());
    assert!(out_dir.join("run_report.json").exists());
    
    // Verify test file is still unchanged
    let content = fs::read_to_string(&test_file).expect("Failed to read test file");
    assert_eq!(content, "original content", "Repository file was modified after write_outputs!");
    
    // Clean up
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_outputs_only_in_out_dir() {
    // Create a temporary directory
    let temp_dir = std::env::temp_dir().join(format!("auto_manager_ai_output_test_{}", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()));
    
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    
    let out_dir = temp_dir.join("outputs");
    
    // Create config
    let config = Config::new(temp_dir.clone(), out_dir.clone());
    
    // Generate plan and write outputs
    let plan = generate_action_plan(&config).expect("Failed to generate plan");
    let report = RunReport::new("test_output".to_string());
    write_outputs(&plan, &report, &out_dir).expect("Failed to write outputs");
    
    // Verify outputs exist in out directory
    assert!(out_dir.exists(), "Output directory not created");
    assert!(out_dir.join("action_plan.json").exists(), "action_plan.json not created");
    assert!(out_dir.join("run_report.json").exists(), "run_report.json not created");
    
    // Verify no files created in repo root
    let repo_files: Vec<_> = fs::read_dir(&temp_dir)
        .expect("Failed to read temp dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();
    
    assert_eq!(repo_files.len(), 0, "Files created in repository root!");
    
    // Clean up
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_read_only_operations() {
    // Create a temporary directory with some files
    let temp_dir = std::env::temp_dir().join(format!("auto_manager_ai_readonly_test_{}", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()));
    
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    
    // Create some test files
    fs::write(temp_dir.join("file1.txt"), "content1").expect("Failed to write file1");
    fs::write(temp_dir.join("file2.txt"), "content2").expect("Failed to write file2");
    
    let out_dir = temp_dir.join("out");
    
    // Create config and generate plan
    let config = Config::new(temp_dir.clone(), out_dir);
    let _plan = generate_action_plan(&config).expect("Failed to generate plan");
    
    // Verify all original files are unchanged
    assert_eq!(
        fs::read_to_string(temp_dir.join("file1.txt")).unwrap(),
        "content1"
    );
    assert_eq!(
        fs::read_to_string(temp_dir.join("file2.txt")).unwrap(),
        "content2"
    );
    
    // Clean up
    fs::remove_dir_all(&temp_dir).ok();
}
