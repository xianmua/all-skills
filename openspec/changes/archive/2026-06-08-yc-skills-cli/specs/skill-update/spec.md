## ADDED Requirements

### Requirement: Update installed skill to latest version
The CLI SHALL allow users to update a previously installed skill to the latest version from its origin repository.

#### Scenario: Update skill to latest version
- **WHEN** user executes `yc-skills update fmt`
- **THEN** the CLI fetches the latest changes from the origin repository
- **AND** updates the skill files to match the latest version
- **AND** displays a success message with the old and new version

#### Scenario: Update non-installed skill
- **WHEN** user executes `yc-skills update nonexistent`
- **AND** the skill is not installed locally
- **THEN** the CLI displays an error message indicating the skill is not installed

### Requirement: Update all installed skills
The CLI SHALL support updating all installed skills at once.

#### Scenario: Update all skills
- **WHEN** user executes `yc-skills update --all`
- **THEN** the CLI iterates through all installed skills
- **AND** updates each one to its latest version
- **AND** reports the status of each update

### Requirement: Check for updates without installing
The CLI SHALL support checking for available updates without downloading them.

#### Scenario: Check update availability
- **WHEN** user executes `yc-skills update fmt --check`
- **THEN** the CLI displays whether an update is available
- **AND** shows the current version and latest version
- **AND** does NOT modify any files

### Requirement: Handle update conflicts
The CLI SHALL handle local modifications to skill files during update.

#### Scenario: Update with local modifications
- **WHEN** user executes `yc-skills update fmt`
- **AND** there are local modifications to the skill files
- **THEN** the CLI SHALL either backup local changes or prompt for resolution
- **AND** use `--force` flag to overwrite local changes