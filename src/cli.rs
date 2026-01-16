use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pdrift")]
#[command(about = "Compare Poetry lock files and detect breaking version bumps")]
pub struct Cli {
    /// Path to the old poetry.lock file
    pub old_lock: PathBuf,

    /// Path to the new poetry.lock file
    pub new_lock: PathBuf,

    /// Output results as JSON
    #[arg(long)]
    pub json: bool,

    /// Include non-breaking changes in the output
    #[arg(long)]
    pub all: bool,
}
