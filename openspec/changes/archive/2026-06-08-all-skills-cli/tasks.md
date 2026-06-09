## 1. Project Setup

- [x] 1.1 Initialize Rust project with `cargo new all-skills`
- [x] 1.2 Add dependencies: clap (CLI), toml (config), git2 (git operations), dirs (paths), reqwest (HTTP)
- [x] 1.3 Setup project structure: src/commands/, src/config/, src/git/, src/models/
- [x] 1.4 Configure Cargo.toml with proper metadata and build settings

## 2. Core Models and Config

- [x] 2.1 Define Skill struct with name, version, description, origin fields
- [x] 2.2 Define Origin struct with name, url, priority fields
- [x] 2.3 Create Config struct for ~/.all-skills/config.toml
- [x] 2.4 Implement Config loading and saving with toml crate
- [x] 2.5 Create default config file creation logic

## 3. Git Operations Module

- [x] 3.1 Implement GitOrigin trait for querying git repositories
- [x] 3.2 Implement skill discovery via git ls-remote
- [x] 3.3 Implement shallow clone with sparse checkout for skill download
- [x] 3.4 Add support for checking skill.yaml/manifest.json existence
- [x] 3.5 Handle git authentication (future: credential helper)

## 4. CLI Commands - Registry

- [x] 4.1 Implement `add-origin` command with URL validation
- [x] 4.2 Implement `list-origins` command
- [x] 4.3 Implement `remove-origin` command
- [x] 4.4 Add origin name/priority configuration

## 5. CLI Commands - Install/Uninstall

- [x] 5.1 Implement `install` command with --trae flag support
- [x] 5.2 Implement skill search across all origins
- [x] 5.3 Implement skill download with progress indicator
- [x] 5.4 Implement `uninstall` command with confirmation prompt
- [x] 5.5 Add --force flag for non-interactive uninstall

## 6. CLI Commands - Update/Search

- [x] 6.1 Implement `update` command to fetch latest from origin
- [x] 6.2 Implement `update --all` for batch update
- [x] 6.3 Implement `update --check` for update check without download
- [x] 6.4 Implement `search` command across origins
- [x] 6.5 Implement `list` command for installed skills

## 7. CLI Framework

- [x] 7.1 Setup clap with subcommand structure
- [x] 7.2 Add global flags: --verbose, --config, --help
- [x] 7.3 Implement colored output and error messages
- [x] 7.4 Add shell completion generation (bash, zsh, fish, powershell)

## 8. Testing

- [x] 8.1 Write unit tests for config loading/saving
- [x] 8.2 Write unit tests for git operations (mock git responses)
- [x] 8.3 Write integration tests for CLI commands
- [x] 8.4 Add example origin config in tests/

## 9. Documentation and Release

- [x] 9.1 Write README.md with installation and usage instructions
- [x] 9.2 Add command help documentation
- [x] 9.3 Configure release workflow (cargo-dist or manual release)
- [x] 9.4 Build binaries for Windows, macOS, Linux