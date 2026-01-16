use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
}

pub fn normalize_name(name: &str) -> String {
    let re = Regex::new(r"[-.]").unwrap();
    re.replace_all(name, "_").to_lowercase()
}

pub fn parse_lockfile(
    path: &Path,
) -> Result<HashMap<String, LockedPackage>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let data: toml::Value = toml::from_str(&content)?;

    let mut packages = HashMap::new();

    if let Some(package_array) = data.get("package").and_then(|v| v.as_array()) {
        for package_entry in package_array {
            let name = package_entry
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or("Missing 'name' field")?
                .to_string();

            let version = package_entry
                .get("version")
                .and_then(|v| v.as_str())
                .ok_or("Missing 'version' field")?
                .to_string();

            let normalized_name = normalize_name(&name);
            packages.insert(
                normalized_name,
                LockedPackage {
                    name: name.clone(),
                    version,
                },
            );
        }
    }

    Ok(packages)
}
