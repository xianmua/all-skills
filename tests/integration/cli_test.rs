//! Integration tests for CLI commands

use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_config_load_and_save() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Create a config
    let mut config = yc_skills::config::Config::default();
    config.add_origin(
        "github".to_string(),
        "https://github.com/test/skills".to_string(),
        None,
    );

    // Save config
    config.save(&config_path).unwrap();

    // Load config
    let loaded = yc_skills::config::Config::load(&config_path).unwrap();

    assert!(loaded.origins.contains_key("github"));
    assert_eq!(
        loaded.origins.get("github").unwrap().url,
        "https://github.com/test/skills"
    );
}

#[test]
fn test_add_and_remove_origin() {
    let mut config = yc_skills::config::Config::default();

    // Add origin
    config.add_origin(
        "gitlab".to_string(),
        "https://gitlab.com/test/skills".to_string(),
        Some(50),
    );

    assert!(config.origins.contains_key("gitlab"));

    // Remove origin
    let removed = config.remove_origin("gitlab");
    assert!(removed);
    assert!(!config.origins.contains_key("gitlab"));

    // Remove non-existent
    let removed_again = config.remove_origin("gitlab");
    assert!(!removed_again);
}

#[test]
fn test_enabled_origins_sorted_by_priority() {
    let mut config = yc_skills::config::Config::default();

    config.add_origin("low".to_string(), "https://low.example.com".to_string(), Some(200));
    config.add_origin("high".to_string(), "https://high.example.com".to_string(), Some(50));
    config.add_origin("mid".to_string(), "https://mid.example.com".to_string(), Some(100));

    let origins = config.get_enabled_origins();

    assert_eq!(origins.len(), 3);
    assert_eq!(origins[0].0, "high");
    assert_eq!(origins[1].0, "mid");
    assert_eq!(origins[2].0, "low");
}