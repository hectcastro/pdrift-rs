use crate::lockfile::LockedPackage;
use pep440_rs::Version;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionBump {
    pub package_name: String,
    pub old_version: String,
    pub new_version: String,
    pub is_breaking: bool,
}

pub fn is_breaking_bump(old_version: &str, new_version: &str) -> bool {
    let old_v: Version = old_version.parse().unwrap();
    let new_v: Version = new_version.parse().unwrap();

    let old_release = old_v.release();
    let new_release = new_v.release();

    // Major version bump is breaking.
    if let (Some(&old_major), Some(&new_major)) = (old_release.first(), new_release.first()) {
        if new_major > old_major {
            return true;
        }

        // 0.x minor bump is breaking.
        if old_major == 0
            && new_major == 0
            && old_release.get(1).unwrap_or(&0) < new_release.get(1).unwrap_or(&0)
        {
            return true;
        }
    }

    false
}

pub fn compare_packages(
    old_packages: &HashMap<String, LockedPackage>,
    new_packages: &HashMap<String, LockedPackage>,
) -> Vec<VersionBump> {
    let mut bumps = Vec::new();

    for (package_name, old_package) in old_packages {
        if let Some(new_package) = new_packages.get(package_name) {
            if old_package.version != new_package.version {
                let is_breaking = is_breaking_bump(&old_package.version, &new_package.version);

                bumps.push(VersionBump {
                    package_name: old_package.name.clone(),
                    old_version: old_package.version.clone(),
                    new_version: new_package.version.clone(),
                    is_breaking,
                });
            }
        }
    }

    bumps.sort_by(|a, b| {
        a.package_name
            .to_lowercase()
            .cmp(&b.package_name.to_lowercase())
    });

    bumps
}
