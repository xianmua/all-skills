## ADDED Requirements

### Requirement: Default origins

The system SHALL provide default origins for common skill repositories.

#### Scenario: Internal GitLab origin included
- **WHEN** default configuration is created
- **THEN** the system SHALL include `http://gitlab.app.yuchai.com/yc90115142/skills.git` as a default origin named "gitlab"
- **AND** its priority SHALL be set to 100