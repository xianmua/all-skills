## ADDED Requirements

### Requirement: Dynamic IDE flag format
The CLI SHALL accept any `--<name>` format as an IDE flag and map it to `.<name>/skills` directory.

#### Scenario: Use --trae flag
- **WHEN** user executes `all-skills install fmt --trae`
- **THEN** the skill is installed to `<current-dir>/.trae/skills/fmt`

#### Scenario: Use --comate flag
- **WHEN** user executes `all-skills install fmt --comate`
- **THEN** the skill is installed to `<current-dir>/.comate/skills/fmt`

#### Scenario: Use --custom flag
- **WHEN** user executes `all-skills install fmt --custom`
- **THEN** the skill is installed to `<current-dir>/.custom/skills/fmt`

#### Scenario: Use --dir conflicts with --xxx
- **WHEN** user executes `all-skills install fmt --trae --dir ./custom`
- **THEN** the CLI displays an error about conflicting options

#### Scenario: Default installation without --xxx
- **WHEN** user executes `all-skills install fmt`
- **THEN** the skill is installed to `<current-dir>/.agent/skills/fmt`
