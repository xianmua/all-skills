## ADDED Requirements

### Requirement: Origin URL validation

The system SHALL validate that origin URLs are well-formed git repository URLs.

#### Scenario: Valid HTTP URL accepted
- **WHEN** user adds origin with URL `https://github.com/org/skills`
- **THEN** the system SHALL accept the URL

#### Scenario: Valid SSH URL accepted
- **WHEN** user adds origin with URL `git@github.com:org/skills`
- **THEN** the system SHALL accept the URL

#### Scenario: Invalid URL rejected
- **WHEN** user adds origin with URL `not-a-url`
- **THEN** the system SHALL reject the URL with an error message

### Requirement: Origin name uniqueness

The system SHALL ensure origin names are unique within the configuration.

#### Scenario: Duplicate origin name rejected
- **WHEN** user attempts to add an origin with a name that already exists
- **THEN** the system SHALL reject the operation with an error message