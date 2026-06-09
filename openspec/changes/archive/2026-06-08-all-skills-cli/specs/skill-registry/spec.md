## ADDED Requirements

### Requirement: Add git origin repository
The CLI SHALL allow users to add a git repository URL as a skill source origin. The origin MUST support GitHub, GitLab, Gitee, and any standard git HTTP/HTTPS URL.

#### Scenario: Add a new GitLab origin
- **WHEN** user executes `all-skills add-origin https://gitlab.internal.company.com/skills`
- **THEN** the origin is added to `~/.all-skills/config.toml` under `[origins]`
- **AND** the CLI displays a success message with the origin URL

#### Scenario: Add origin with custom name
- **WHEN** user executes `all-skills add-origin https://gitlab.internal.company.com/skills --name internal`
- **THEN** the origin is stored with key `internal` in the config file

#### Scenario: Add duplicate origin
- **WHEN** user adds an origin that already exists
- **THEN** the CLI SHALL display an error message indicating the origin already exists
- **AND** the existing configuration SHALL NOT be modified

### Requirement: List configured origins
The CLI SHALL allow users to view all configured git origins.

#### Scenario: List all origins
- **WHEN** user executes `all-skills list-origins`
- **THEN** the CLI displays all configured origins with their URLs and names

#### Scenario: List origins when none configured
- **WHEN** user executes `all-skills list-origins` with no origins configured
- **THEN** the CLI displays a message indicating no origins are configured

### Requirement: Remove git origin repository
The CLI SHALL allow users to remove a configured git origin.

#### Scenario: Remove an existing origin
- **WHEN** user executes `all-skills remove-origin internal`
- **THEN** the origin with name `internal` is removed from config
- **AND** a success message is displayed

#### Scenario: Remove non-existent origin
- **WHEN** user executes `all-skills remove-origin nonexistent`
- **THEN** the CLI displays an error message indicating the origin was not found

### Requirement: Origin configuration persistence
The CLI SHALL persist origin configurations to `~/.all-skills/config.toml` and reload on startup.

#### Scenario: Config file created on first use
- **WHEN** user adds an origin for the first time
- **THEN** the CLI creates `~/.all-skills/` directory if it does not exist
- **AND** creates `config.toml` with the origin configuration

#### Scenario: Config file preserves existing origins
- **WHEN** user adds a new origin while other origins exist
- **THEN** all existing origins SHALL be preserved
- **AND** the new origin is appended to the configuration