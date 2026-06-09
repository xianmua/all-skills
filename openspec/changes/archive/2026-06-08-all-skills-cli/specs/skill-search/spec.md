## ADDED Requirements

### Requirement: Search for available skills
The CLI SHALL allow users to search for skills across all configured git origins.

#### Scenario: Search skills by keyword
- **WHEN** user executes `all-skills search fmt`
- **THEN** the CLI searches all configured origins for skills matching `fmt`
- **AND** displays a list of matching skills with their descriptions

#### Scenario: Search with no results
- **WHEN** user executes `all-skills search nonexistent-keyword`
- **AND** no skills match the keyword
- **THEN** the CLI displays a message indicating no skills were found

### Requirement: List installed skills
The CLI SHALL allow users to list all skills installed locally.

#### Scenario: List installed skills
- **WHEN** user executes `all-skills list`
- **THEN** the CLI displays all locally installed skills
- **AND** shows each skill's name, version, and installation path

#### Scenario: List installed skills when none exist
- **WHEN** user executes `all-skills list`
- **AND** no skills are installed
- **THEN** the CLI displays a message indicating no skills are installed

### Requirement: Search results format
The CLI SHALL display search results in a human-readable format with skill metadata.

#### Scenario: Display skill search results
- **WHEN** user executes `all-skills search openspec`
- **THEN** the CLI displays results in a table format with columns: Name, Description, Origin, Version
- **AND** displays up to 20 results by default
- **AND** supports `--limit` flag to change the number of results