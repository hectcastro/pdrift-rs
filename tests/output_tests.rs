use pdrift_rs::compare::VersionBump;
use pdrift_rs::output::{format_json, format_text};

// Tests for format_text

#[test]
fn test_formats_breaking_change() {
    let bumps = vec![VersionBump {
        package_name: "pkg".to_string(),
        old_version: "1.0.0".to_string(),
        new_version: "2.0.0".to_string(),
        is_breaking: true,
    }];

    let result = format_text(&bumps, false);

    assert!(result.contains("pkg"));
    assert!(result.contains("1.0.0"));
    assert!(result.contains("2.0.0"));
}

#[test]
fn test_formats_multiple_breaking_changes() {
    let bumps = vec![
        VersionBump {
            package_name: "pkg1".to_string(),
            old_version: "1.0.0".to_string(),
            new_version: "2.0.0".to_string(),
            is_breaking: true,
        },
        VersionBump {
            package_name: "pkg2".to_string(),
            old_version: "0.5.0".to_string(),
            new_version: "0.6.0".to_string(),
            is_breaking: true,
        },
    ];

    let result = format_text(&bumps, false);

    assert!(result.contains("pkg1"));
    assert!(result.contains("pkg2"));
    assert!(result.contains("2 breaking"));
}

#[test]
fn test_shows_count() {
    let bumps = vec![
        VersionBump {
            package_name: "pkg1".to_string(),
            old_version: "1.0.0".to_string(),
            new_version: "2.0.0".to_string(),
            is_breaking: true,
        },
        VersionBump {
            package_name: "pkg2".to_string(),
            old_version: "2.0.0".to_string(),
            new_version: "3.0.0".to_string(),
            is_breaking: true,
        },
    ];

    let result = format_text(&bumps, false);

    assert!(result.contains("2 breaking"));
}

#[test]
fn test_empty_when_no_breaking_changes() {
    let bumps: Vec<VersionBump> = vec![];

    let result = format_text(&bumps, false);

    assert!(result.to_lowercase().contains("no breaking") || result.trim().is_empty());
}

// Tests for format_json

#[test]
fn test_valid_json_output() {
    let bumps = vec![VersionBump {
        package_name: "pkg".to_string(),
        old_version: "1.0.0".to_string(),
        new_version: "2.0.0".to_string(),
        is_breaking: true,
    }];

    let result = format_json(&bumps, false);

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed.is_object());
}

#[test]
fn test_contains_breaking_changes_key() {
    let bumps = vec![VersionBump {
        package_name: "pkg".to_string(),
        old_version: "1.0.0".to_string(),
        new_version: "2.0.0".to_string(),
        is_breaking: true,
    }];

    let result = format_json(&bumps, false);

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed.get("breaking_changes").is_some());
}

#[test]
fn test_includes_package_details() {
    let bumps = vec![VersionBump {
        package_name: "pkg".to_string(),
        old_version: "1.0.0".to_string(),
        new_version: "2.0.0".to_string(),
        is_breaking: true,
    }];

    let result = format_json(&bumps, false);

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let change = &parsed["breaking_changes"][0];
    assert_eq!(change["package"], "pkg");
    assert_eq!(change["old_version"], "1.0.0");
    assert_eq!(change["new_version"], "2.0.0");
}

#[test]
fn test_all_flag_includes_non_breaking() {
    let bumps = vec![
        VersionBump {
            package_name: "pkg1".to_string(),
            old_version: "1.0.0".to_string(),
            new_version: "2.0.0".to_string(),
            is_breaking: true,
        },
        VersionBump {
            package_name: "pkg2".to_string(),
            old_version: "1.0.0".to_string(),
            new_version: "1.1.0".to_string(),
            is_breaking: false,
        },
    ];

    let result = format_json(&bumps, true);

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["breaking_changes"].as_array().unwrap().len(), 1);
    assert!(parsed.get("non_breaking_changes").is_some());
    assert_eq!(parsed["non_breaking_changes"].as_array().unwrap().len(), 1);
}

#[test]
fn test_all_false_filters_non_breaking() {
    let bumps = vec![
        VersionBump {
            package_name: "pkg1".to_string(),
            old_version: "1.0.0".to_string(),
            new_version: "2.0.0".to_string(),
            is_breaking: true,
        },
        VersionBump {
            package_name: "pkg2".to_string(),
            old_version: "1.0.0".to_string(),
            new_version: "1.1.0".to_string(),
            is_breaking: false,
        },
    ];

    let result = format_json(&bumps, false);

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["breaking_changes"].as_array().unwrap().len(), 1);
    assert!(parsed.get("non_breaking_changes").is_none());
}
