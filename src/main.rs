use clap::Parser;
use pdrift_rs::cli::Cli;
use pdrift_rs::compare::compare_packages;
use pdrift_rs::lockfile::parse_lockfile;
use pdrift_rs::output::{format_json, format_text};
use std::process;

fn main() {
    let args = Cli::parse();

    let old_packages = match parse_lockfile(&args.old_lock) {
        Ok(packages) => packages,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(2);
        }
    };

    let new_packages = match parse_lockfile(&args.new_lock) {
        Ok(packages) => packages,
        Err(e) => {
            eprintln!("Error parsing lock files: {}", e);
            process::exit(2);
        }
    };

    let bumps = compare_packages(&old_packages, &new_packages);

    let output = if args.json {
        format_json(&bumps, args.all)
    } else {
        format_text(&bumps, args.all)
    };

    println!("{}", output);

    let breaking_bumps: Vec<_> = bumps.iter().filter(|b| b.is_breaking).collect();
    if !breaking_bumps.is_empty() {
        process::exit(1);
    }
}
