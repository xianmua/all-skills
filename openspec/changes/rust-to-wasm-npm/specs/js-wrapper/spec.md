## ADDED Requirements

### Requirement: JavaScript API wrapper

The system SHALL provide a JavaScript/TypeScript API wrapper that exposes core functionality.

#### Scenario: Async initialization
- **WHEN** application imports the wrapper
- **THEN** the WASM module SHALL initialize asynchronously before use

#### Scenario: Core functions exported
- **WHEN** WASM module is loaded
- **THEN** the following functions SHALL be exported:
- `installSkill(name: string, options?: InstallOptions): Promise<void>`
- `uninstallSkill(name: string): Promise<void>`
- `updateSkill(name: string): Promise<void>`
- `listSkills(): Promise<Skill[]>`
- `searchSkills(query: string): Promise<SearchResult[]>`

### Requirement: Error handling

The JavaScript wrapper SHALL provide user-friendly error messages in JavaScript domain.

#### Scenario: Graceful error on missing skill
- **WHEN** `installSkill` is called with non-existent skill name
- **THEN** a JavaScript Error with descriptive message SHALL be thrown

#### Scenario: Network error handling
- **WHEN** network request fails during operation
- **THEN** a JavaScript Error with network details SHALL be thrown