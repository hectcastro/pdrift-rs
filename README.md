# pdrift-rs

A command-line tool to compare Poetry lock files and detect breaking version bumps.

- **Major version bumps** (1.x.x → 2.x.x) are considered breaking
- **0.x minor bumps** (0.9.x → 0.10.x) are considered breaking
- Patch bumps and minor bumps for stable versions (≥1.0.0) are non-breaking

## Usage

```bash
Usage: pdrift [OPTIONS] <OLD_LOCK> <NEW_LOCK>

Arguments:
  <OLD_LOCK>  Path to the old poetry.lock file
  <NEW_LOCK>  Path to the new poetry.lock file

Options:
      --json  Output results as JSON
      --all   Include non-breaking changes in the output
  -h, --help  Print help
```

## Examples

```bash
# Basic usage
pdrift poetry-old.lock poetry-new.lock

# JSON output
pdrift poetry-old.lock poetry-new.lock --json

# Include all changes, not just major version changes
pdrift poetry-old.lock poetry-new.lock --all
```

## Development

This project uses [Mise](https://mise.jdx.dev/) for tool version management.

```bash
# Install dependencies
mise install

# Run tests
make test

# Run linter
make lint

# Format code
make format

# Run full CI pipeline
make ci
```
