use pdrift_rs::compare::{compare_packages, is_breaking_bump};
use pdrift_rs::lockfile::LockedPackage;
use std::collections::HashMap;

// Tests for is_breaking_bump

#[test]
fn test_breaking_major_version_bump() {
    assert!(is_breaking_bump("1.0.0", "2.0.0"));
    assert!(is_breaking_bump("1.5.0", "2.0.0"));
    assert!(is_breaking_bump("2.0.0", "3.0.0"));
}

#[test]
fn test_breaking_0x_minor_bump() {
    assert!(is_breaking_bump("0.9.0", "0.10.0"));
    assert!(is_breaking_bump("0.1.0", "0.2.0"));
}

#[test]
fn test_breaking_0x_to_1x() {
    assert!(is_breaking_bump("0.9.9", "1.0.0"));
}

#[test]
fn test_breaking_prerelease_versions() {
    assert!(is_breaking_bump("1.0.0a1", "2.0.0"));
    assert!(is_breaking_bump("1.0.0", "2.0.0a1"));
}

#[test]
fn test_breaking_epoch_major_bump() {
    assert!(is_breaking_bump("1!1.0.0", "1!2.0.0"));
}

#[test]
fn test_non_breaking_minor_bump_1x() {
    assert!(!is_breaking_bump("1.1.0", "1.2.0"));
    assert!(!is_breaking_bump("2.0.0", "2.1.0"));
}

#[test]
fn test_non_breaking_patch_bump() {
    assert!(!is_breaking_bump("1.0.0", "1.0.1"));
    assert!(!is_breaking_bump("2.5.1", "2.5.2"));
}

#[test]
fn test_non_breaking_0x_patch_bump() {
    assert!(!is_breaking_bump("0.9.0", "0.9.1"));
    assert!(!is_breaking_bump("0.1.0", "0.1.5"));
}

#[test]
fn test_non_breaking_same_version() {
    assert!(!is_breaking_bump("1.0.0", "1.0.0"));
    assert!(!is_breaking_bump("0.5.0", "0.5.0"));
}

#[test]
fn test_non_breaking_downgrade() {
    assert!(!is_breaking_bump("2.0.0", "1.0.0"));
    assert!(!is_breaking_bump("0.5.0", "0.4.0"));
}

// Tests for compare_packages

#[test]
fn test_detects_version_bump() {
    let mut old = HashMap::new();
    old.insert(
        "urllib3".to_string(),
        LockedPackage {
            name: "urllib3".to_string(),
            version: "1.26.0".to_string(),
        },
    );

    let mut new = HashMap::new();
    new.insert(
        "urllib3".to_string(),
        LockedPackage {
            name: "urllib3".to_string(),
            version: "2.0.0".to_string(),
        },
    );

    let bumps = compare_packages(&old, &new);

    assert_eq!(bumps.len(), 1);
    assert_eq!(bumps[0].package_name, "urllib3");
    assert_eq!(bumps[0].old_version, "1.26.0");
    assert_eq!(bumps[0].new_version, "2.0.0");
    assert!(bumps[0].is_breaking);
}

#[test]
fn test_detects_non_breaking_bump() {
    let mut old = HashMap::new();
    old.insert(
        "requests".to_string(),
        LockedPackage {
            name: "requests".to_string(),
            version: "2.31.0".to_string(),
        },
    );

    let mut new = HashMap::new();
    new.insert(
        "requests".to_string(),
        LockedPackage {
            name: "requests".to_string(),
            version: "2.32.0".to_string(),
        },
    );

    let bumps = compare_packages(&old, &new);

    assert_eq!(bumps.len(), 1);
    assert_eq!(bumps[0].package_name, "requests");
    assert!(!bumps[0].is_breaking);
}

#[test]
fn test_handles_new_package() {
    let old: HashMap<String, LockedPackage> = HashMap::new();

    let mut new = HashMap::new();
    new.insert(
        "newpkg".to_string(),
        LockedPackage {
            name: "newpkg".to_string(),
            version: "1.0.0".to_string(),
        },
    );

    let bumps = compare_packages(&old, &new);

    assert_eq!(bumps.len(), 0);
}

#[test]
fn test_handles_removed_package() {
    let mut old = HashMap::new();
    old.insert(
        "oldpkg".to_string(),
        LockedPackage {
            name: "oldpkg".to_string(),
            version: "1.0.0".to_string(),
        },
    );

    let new: HashMap<String, LockedPackage> = HashMap::new();

    let bumps = compare_packages(&old, &new);

    assert_eq!(bumps.len(), 0);
}

#[test]
fn test_handles_same_version() {
    let mut old = HashMap::new();
    old.insert(
        "pkg".to_string(),
        LockedPackage {
            name: "pkg".to_string(),
            version: "1.0.0".to_string(),
        },
    );

    let mut new = HashMap::new();
    new.insert(
        "pkg".to_string(),
        LockedPackage {
            name: "pkg".to_string(),
            version: "1.0.0".to_string(),
        },
    );

    let bumps = compare_packages(&old, &new);

    assert_eq!(bumps.len(), 0);
}

#[test]
fn test_multiple_packages() {
    let mut old = HashMap::new();
    old.insert(
        "pkg1".to_string(),
        LockedPackage {
            name: "pkg1".to_string(),
            version: "1.0.0".to_string(),
        },
    );
    old.insert(
        "pkg2".to_string(),
        LockedPackage {
            name: "pkg2".to_string(),
            version: "2.0.0".to_string(),
        },
    );
    old.insert(
        "pkg3".to_string(),
        LockedPackage {
            name: "pkg3".to_string(),
            version: "3.0.0".to_string(),
        },
    );

    let mut new = HashMap::new();
    new.insert(
        "pkg1".to_string(),
        LockedPackage {
            name: "pkg1".to_string(),
            version: "2.0.0".to_string(),
        },
    );
    new.insert(
        "pkg2".to_string(),
        LockedPackage {
            name: "pkg2".to_string(),
            version: "2.1.0".to_string(),
        },
    );
    new.insert(
        "pkg3".to_string(),
        LockedPackage {
            name: "pkg3".to_string(),
            version: "3.0.0".to_string(),
        },
    );

    let bumps = compare_packages(&old, &new);

    assert_eq!(bumps.len(), 2);
    let pkg1_bump = bumps.iter().find(|b| b.package_name == "pkg1").unwrap();
    assert!(pkg1_bump.is_breaking);
    let pkg2_bump = bumps.iter().find(|b| b.package_name == "pkg2").unwrap();
    assert!(!pkg2_bump.is_breaking);
}

#[test]
fn test_sorted_by_package_name() {
    let mut old = HashMap::new();
    old.insert(
        "zebra".to_string(),
        LockedPackage {
            name: "zebra".to_string(),
            version: "1.0.0".to_string(),
        },
    );
    old.insert(
        "alpha".to_string(),
        LockedPackage {
            name: "alpha".to_string(),
            version: "1.0.0".to_string(),
        },
    );
    old.insert(
        "beta".to_string(),
        LockedPackage {
            name: "beta".to_string(),
            version: "1.0.0".to_string(),
        },
    );

    let mut new = HashMap::new();
    new.insert(
        "zebra".to_string(),
        LockedPackage {
            name: "zebra".to_string(),
            version: "2.0.0".to_string(),
        },
    );
    new.insert(
        "alpha".to_string(),
        LockedPackage {
            name: "alpha".to_string(),
            version: "2.0.0".to_string(),
        },
    );
    new.insert(
        "beta".to_string(),
        LockedPackage {
            name: "beta".to_string(),
            version: "2.0.0".to_string(),
        },
    );

    let bumps = compare_packages(&old, &new);

    let names: Vec<&str> = bumps.iter().map(|b| b.package_name.as_str()).collect();
    assert_eq!(names, vec!["alpha", "beta", "zebra"]);
}
