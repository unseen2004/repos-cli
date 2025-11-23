# Repos CLI

A Rust-based CLI tool to manage GitHub repositories and automatically update README.md files with dynamic content.

## Features

- **Authentication**: Securely login with GitHub PAT.
- **Repository Management**: List your GitHub repositories.
- **Auto-Updates**: Automatically update READMEs with:
    - Tech Stack Badges
    - Dependency Lists
    - Docker Build Commands
    - Recent Changelogs
    - Sub-project Structure (for monorepos)
- **Recursive**: Updates all sub-projects in a directory tree.

## Installation

```bash
cargo install --path .
```

## Usage

1. **Login**:
   ```bash
   repos-cli auth login
   ```

2. **Track a Repository**:
   ```bash
   repos-cli add /path/to/repo
   ```

3. **Update READMEs**:
   ```bash
   repos-cli update
   ```

## Configuration

Global configuration is stored in your system's config directory (e.g., `~/.config/repos-cli` on Linux).
Local configuration can be added to repositories via `.repos-cli.toml`.
