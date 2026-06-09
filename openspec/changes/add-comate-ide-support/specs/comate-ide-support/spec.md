## ADDED Requirements

### Requirement: Install skill to Comate IDE directory
The CLI SHALL allow users to install a skill to the `.comate/skills` directory by using the `--comate` flag.

#### Scenario: Install skill with --comate flag
- **WHEN** user executes `yc-skills install fmt --comate`
- **THEN** the CLI downloads the skill to `<current-dir>/.comate/skills/fmt/`

#### Scenario: --comate conflicts with --trae
- **WHEN** user executes `yc-skills install fmt --comate --trae`
- **THEN** the CLI displays an error about conflicting options
- **AND** neither option is applied

#### Scenario: --comate conflicts with --clion
- **WHEN** user executes `yc-skills install fmt --comate --clion`
- **THEN** the CLI displays an error about conflicting options
- **AND** neither option is applied

#### Scenario: --comate conflicts with --dir
- **WHEN** user executes `yc-skills install fmt --comate --dir ./custom`
- **THEN** the CLI displays an error about conflicting options
- **AND** neither option is applied

### Requirement: Default IDE remains trae
The CLI SHALL default to `--trae` when no IDE flag is specified.

#### Scenario: Install without IDE flag defaults to trae
- **WHEN** user executes `yc-skills install fmt`
- **THEN** the skill is installed to `<current-dir>/.trae/skills/fmt/`
- **AND** a message indicates the default IDE is being used