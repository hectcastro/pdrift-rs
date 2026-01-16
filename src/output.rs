use crate::compare::VersionBump;
use serde::Serialize;

pub fn format_text(bumps: &[VersionBump], all: bool) -> String {
    let breaking_bumps: Vec<&VersionBump> = bumps.iter().filter(|b| b.is_breaking).collect();
    let non_breaking_bumps: Vec<&VersionBump> = bumps.iter().filter(|b| !b.is_breaking).collect();

    if breaking_bumps.is_empty() && non_breaking_bumps.is_empty() {
        return "No breaking changes detected.".to_string();
    }
    if breaking_bumps.is_empty() && !all {
        return "No breaking changes detected.".to_string();
    }

    let mut lines = Vec::new();

    if !breaking_bumps.is_empty() {
        lines.push("Breaking changes detected:".to_string());
        for bump in &breaking_bumps {
            let change_type = if bump.new_version.starts_with('0') {
                "0.x MINOR"
            } else {
                "MAJOR"
            };
            let version_info = format!("{} → {}", bump.old_version, bump.new_version);
            lines.push(format!(
                "  {}: {} ({})",
                bump.package_name, version_info, change_type
            ));
        }
        lines.push(format!(
            "{} breaking change(s) found.",
            breaking_bumps.len()
        ));
    }

    if all && !non_breaking_bumps.is_empty() {
        if !breaking_bumps.is_empty() {
            lines.push(String::new());
        }
        lines.push("Non-breaking changes:".to_string());
        for bump in &non_breaking_bumps {
            lines.push(format!(
                "  {}: {} → {}",
                bump.package_name, bump.old_version, bump.new_version
            ));
        }
    }

    lines.join("\n")
}

#[derive(Serialize)]
struct PackageChange {
    package: String,
    old_version: String,
    new_version: String,
}

pub fn format_json(bumps: &[VersionBump], all: bool) -> String {
    let breaking_bumps: Vec<&VersionBump> = bumps.iter().filter(|b| b.is_breaking).collect();
    let non_breaking_bumps: Vec<&VersionBump> = bumps.iter().filter(|b| !b.is_breaking).collect();

    let breaking_changes: Vec<PackageChange> = breaking_bumps
        .iter()
        .map(|b| PackageChange {
            package: b.package_name.clone(),
            old_version: b.old_version.clone(),
            new_version: b.new_version.clone(),
        })
        .collect();

    let mut result = serde_json::json!({
        "breaking_changes": breaking_changes
    });

    if all && !non_breaking_bumps.is_empty() {
        let non_breaking_changes: Vec<PackageChange> = non_breaking_bumps
            .iter()
            .map(|b| PackageChange {
                package: b.package_name.clone(),
                old_version: b.old_version.clone(),
                new_version: b.new_version.clone(),
            })
            .collect();
        result["non_breaking_changes"] = serde_json::json!(non_breaking_changes);
    }

    serde_json::to_string_pretty(&result).unwrap()
}
