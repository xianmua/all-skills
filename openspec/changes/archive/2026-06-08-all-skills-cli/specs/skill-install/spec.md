## ADDED Requirements

### Requirement: Install skill from git repository
The CLI SHALL allow users to install a skill by name from configured git repositories. The CLI SHALL search through origins in priority order until the skill is found.

#### Scenario: Install skill with --trae flag
- **WHEN** user executes `all-skills install fmt --trae`
- **THEN** the CLI searches for `fmt` skill in configured origins
- **AND** downloads the skill to `<current-dir>/.trae/skills/fmt/`

#### Scenario: Install skill that does not exist
- **WHEN** user executes `all-skills install nonexistent`
- **AND** the skill is not found in any configured origin
- **THEN** the CLI displays an error message indicating the skill was not found
- **AND** no files are created

#### Scenario: Install skill that is already installed
- **WHEN** user executes `all-skills install fmt --trae`
- **AND** the skill `fmt` is already installed in the target directory
- **THEN** the CLI SHALL prompt for confirmation or use `--force` flag to overwrite

### Requirement: Skill discovery from git repository
The CLI SHALL detect skill existence by checking for `skill.yaml` or `manifest.json` file in the repository at path `<skill-name>/`.

#### Scenario: Discover skill via skill.yaml
- **WHEN** the CLI queries a git origin for skill `fmt`
- **THEN** it checks if `fmt/skill.yaml` exists in the repository
- **AND** if found, considers the skill as available

#### Scenario: Discover skill via manifest.json
- **WHEN** the CLI queries a git origin for skill `fmt`
- **THEN** it checks if `fmt/manifest.json` exists as fallback
- **AND** if found, considers the skill as available

### Requirement: Git clone strategy
The CLI SHALL use shallow clone to minimize bandwidth usage when downloading skills.

#### Scenario: Shallow clone for installation
- **WHEN** user installs a skill
- **THEN** the CLI uses `git clone --depth 1` to clone only the latest commit
- **AND** clones only the specific skill directory using sparse checkout

#### Scenario: Clone specific subdirectory
- **WHEN** installing skill `fmt`
- **THEN** the CLI clones only `fmt/` subdirectory from the repository
- **AND** excludes other skill directories

### Requirement: IDE-specific installation directory
The CLI SHALL support installing skills to IDE-specific directories based on CLI flags.

#### Scenario: Install to .trae/skills directory
- **WHEN** user executes `all-skills install fmt --trae`
- **THEN** the skill is installed to `<current-dir>/.trae/skills/fmt/`

#### Scenario: Custom installation directory
- **WHEN** user executes `all-skills install fmt --dir ./custom-skills`
- **THEN** the skill is installed to `./custom-skills/fmt/`