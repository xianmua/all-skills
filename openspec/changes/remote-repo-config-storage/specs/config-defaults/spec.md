## ADDED Requirements

### Requirement: Default origins

The system SHALL provide default origins for common skill repositories.

#### Scenario: Default GitHub origin included
- **WHEN** default configuration is created
- **THEN** the system SHALL include `https://github.com/xianmua/skills.git` as a default origin named "github"
- **AND** its priority SHALL be set to 100