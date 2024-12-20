use eliza_cli::config::Config;
use eliza_cli::error::ElizaCliError;
use std::fs;
use std::io::Write;

#[test]
fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.log_level, "info");
    assert_eq!(config.data_path, "./data");
}

#[test]
fn test_load_config_success() {
    let tmp_file = "test_config.json";
    let json = r#"{ "log_level": "debug", "data_path": "/tmp/data" }"#;
    fs::write(tmp_file, json).unwrap();

    let config = Config::load_from_file(tmp_file).unwrap();
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.data_path, "/tmp/data");

    fs::remove_file(tmp_file).unwrap();
}

#[test]
fn test_load_config_failure() {
    let result = Config::load_from_file("non_existent.json");
    assert!(result.is_err());

    match result.err().unwrap() {
        ElizaCliError::ConfigError(msg) => {
            assert!(msg.contains("Failed to read config"));
        }
        _ => panic!("Unexpected error type"),
    }
}
 