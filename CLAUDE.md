# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

pdrift-rs is a Rust CLI tool that compares Poetry lock files to detect breaking version bumps. It identifies:
- Major version bumps (1.x.x → 2.x.x) as breaking
- 0.x minor bumps (0.9.x → 0.10.x) as breaking
- Patch and minor bumps for stable versions (≥1.0.0) as non-breaking

## Development Commands

Tool management uses [Mise](https://mise.jdx.dev/). All commands are in the Makefile:

```bash
# Run tests
make test

# Run linter (clippy with warnings as errors)
make lint

# Format code
make format

# Full CI pipeline (lint + test)
make ci

# Build release binary
make build
```

### Running Individual Tests

```bash
# Run a specific test file
mise exec -- cargo test --test lockfile_tests

# Run a specific test function
mise exec -- cargo test test_function_name

# Run with output shown
mise exec -- cargo test -- --nocapture
```

## Code Architecture

### Module Structure

The codebase follows a simple pipeline architecture:

1. **cli.rs** - Command-line argument parsing using clap
2. **lockfile.rs** - Parses Poetry lock files (TOML) into `HashMap<String, LockedPackage>`
   - Normalizes package names (handles `-` and `.` variations)
3. **compare.rs** - Compares two lockfile HashMaps to produce `Vec<VersionBump>`
   - Uses pep440_rs for version parsing
   - `is_breaking_bump()` determines if a version change is breaking
4. **output.rs** - Formats results as text or JSON

### Data Flow

```
poetry.lock files → parse_lockfile() → HashMap<String, LockedPackage>
                                              ↓
                                       compare_packages()
                                              ↓
                                       Vec<VersionBump>
                                              ↓
                                    format_text() or format_json()
```

### Key Types

- `LockedPackage`: Represents a package with name and version
- `VersionBump`: Represents a version change with breaking status
- Package names are normalized to handle Python naming conventions

### Testing

Tests are organized by module behavior:
- `lockfile_tests.rs` - Parsing and name normalization
- `compare_tests.rs` - Version comparison logic
- `output_tests.rs` - Output formatting
- `tests/fixtures/` - Sample Poetry lock files for integration tests

Use test fixtures for integration tests rather than creating lock files inline.
