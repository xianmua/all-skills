## ADDED Requirements

### Requirement: Uninstall installed skill
The CLI SHALL allow users to uninstall a previously installed skill by removing its directory and cleaning up related files.

#### Scenario: Uninstall existing skill
- **WHEN** user executes `yc-skills uninstall fmt`
- **THEN** the CLI removes `<current-dir>/.trae/skills/fmt/` directory
- **AND** displays a success message

#### Scenario: Uninstall non-installed skill
- **WHEN** user executes `yc-skills uninstall nonexistent`
- **AND** the skill is not installed in any search path
- **THEN** the CLI displays an error message indicating the skill was not found

### Requirement: Uninstall with confirmation
The CLI SHALL prompt for confirmation before removing a skill unless `--force` flag is provided.

#### Scenario: Uninstall without force flag
- **WHEN** user executes `yc-skills uninstall fmt`
- **THEN** the CLI prompts for confirmation
- **AND** waits for user input before proceeding

#### Scenario: Uninstall with force flag
- **WHEN** user executes `yc-skills uninstall fmt --force`
- **THEN** the CLI removes the skill directory immediately without confirmation

### Requirement: Clean up skill metadata
The CLI SHALL clean up any skill metadata stored in local configuration when uninstalling.

#### Scenario: Clean local cache on uninstall
- **WHEN** user uninstalls a skill
- **THEN** the CLI removes any cached information about the skill
- **AND** removes the skill from the local installed skills list