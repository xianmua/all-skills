## ADDED Requirements

### Requirement: Configuration file location

The system SHALL store configuration at `~/.all-skills/config.toml` using the `dirs` crate to determine the user home directory.

#### Scenario: Config directory creation
- **WHEN** configuration file is first accessed
- **THEN** the system SHALL create `~/.all-skills/` directory if it does not exist

### Requirement: Configuration persistence

The system SHALL persist all configuration changes to the config file atomically.

#### Scenario: Save configuration after adding origin
- **WHEN** user runs `all-skills add-origin https://example.com/skills`
- **THEN** the updated configuration SHALL be written to `~/.all-skills/config.toml`

#### Scenario: Load configuration on startup
- **WHEN** all-skills starts
- **THEN** it SHALL load configuration from `~/.all-skills/config.toml`

### Requirement: Default configuration initialization

The system SHALL create a default configuration with a built-in GitLab origin if no configuration file exists.

#### Scenario: First run creates default config
- **WHEN** all-skills starts with no existing configuration
- **THEN** a default configuration SHALL be created at `~/.all-skills/config.toml`
- **AND** it SHALL include the default internal GitLab origin