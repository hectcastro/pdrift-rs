use pdrift_rs::lockfile::{normalize_name, parse_lockfile};
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_normalize_lowercase() {
    assert_eq!(normalize_name("Django"), "django");
    assert_eq!(normalize_name("REQUESTS"), "requests");
}

#[test]
fn test_normalize_hyphen_to_underscore() {
    assert_eq!(normalize_name("some-package"), "some_package");
    assert_eq!(normalize_name("flask-cors"), "flask_cors");
}

#[test]
fn test_normalize_period_to_underscore() {
    assert_eq!(normalize_name("zope.interface"), "zope_interface");
}

#[test]
fn test_normalize_already_normalized() {
    assert_eq!(normalize_name("requests"), "requests");
    assert_eq!(normalize_name("pytest"), "pytest");
}

#[test]
fn test_normalize_mixed_case_and_separators() {
    assert_eq!(normalize_name("Some-Package.Name"), "some_package_name");
}

#[test]
fn test_parse_simple_lockfile() {
    let tmp_dir = TempDir::new().unwrap();
    let lock_file = tmp_dir.path().join("poetry.lock");

    let lock_content = r#"[[package]]
name = "requests"
version = "2.31.0"
description = "Python HTTP library"
"#;

    std::fs::write(&lock_file, lock_content).unwrap();

    let result = parse_lockfile(&lock_file).unwrap();

    assert!(result.contains_key("requests"));
    assert_eq!(result["requests"].name, "requests");
    assert_eq!(result["requests"].version, "2.31.0");
}

#[test]
fn test_parse_multiple_packages() {
    let fixture_path = Path::new("tests/fixtures/poetry-old.lock");
    let result = parse_lockfile(fixture_path).unwrap();

    assert_eq!(result.len(), 5);
    assert!(result.contains_key("requests"));
    assert!(result.contains_key("urllib3"));
    assert!(result.contains_key("numpy"));
}

#[test]
fn test_normalize_package_names() {
    let tmp_dir = TempDir::new().unwrap();
    let lock_file = tmp_dir.path().join("poetry.lock");

    let lock_content = r#"[[package]]
name = "Django-CORS-Headers"
version = "4.0.0"
"#;

    std::fs::write(&lock_file, lock_content).unwrap();

    let result = parse_lockfile(&lock_file).unwrap();

    assert!(result.contains_key("django_cors_headers"));
    assert_eq!(result["django_cors_headers"].name, "Django-CORS-Headers");
}

#[test]
fn test_file_not_found() {
    let result = parse_lockfile(Path::new("/nonexistent/poetry.lock"));
    assert!(result.is_err());
}

#[test]
fn test_invalid_toml() {
    let tmp_dir = TempDir::new().unwrap();
    let lock_file = tmp_dir.path().join("poetry.lock");

    std::fs::write(&lock_file, "this is not valid TOML {{{").unwrap();

    let result = parse_lockfile(&lock_file);
    assert!(result.is_err());
}

#[test]
fn test_empty_lockfile() {
    let tmp_dir = TempDir::new().unwrap();
    let lock_file = tmp_dir.path().join("poetry.lock");

    std::fs::write(&lock_file, "").unwrap();

    let result = parse_lockfile(&lock_file).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_lockfile_without_packages() {
    let tmp_dir = TempDir::new().unwrap();
    let lock_file = tmp_dir.path().join("poetry.lock");

    let lock_content = r#"[metadata]
lock-version = "2.0"
python-versions = "^3.12"
"#;

    std::fs::write(&lock_file, lock_content).unwrap();

    let result = parse_lockfile(&lock_file).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_missing_name_field() {
    let tmp_dir = TempDir::new().unwrap();
    let lock_file = tmp_dir.path().join("poetry.lock");

    let lock_content = r#"[[package]]
version = "1.0.0"
"#;

    std::fs::write(&lock_file, lock_content).unwrap();

    let result = parse_lockfile(&lock_file);
    assert!(result.is_err());
}

#[test]
fn test_missing_version_field() {
    let tmp_dir = TempDir::new().unwrap();
    let lock_file = tmp_dir.path().join("poetry.lock");

    let lock_content = r#"[[package]]
name = "mypackage"
"#;

    std::fs::write(&lock_file, lock_content).unwrap();

    let result = parse_lockfile(&lock_file);
    assert!(result.is_err());
}
