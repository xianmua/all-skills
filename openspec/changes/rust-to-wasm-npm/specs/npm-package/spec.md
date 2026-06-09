## ADDED Requirements

### Requirement: NPM package structure

The system SHALL produce a valid NPM package with proper metadata in package.json.

#### Scenario: Package has required fields
- **WHEN** package is published to npm
- **THEN** it SHALL contain: name, version, description, main, module, types, exports, files

#### Scenario: ESM/CJS dual format
- **WHEN** package is installed
- **THEN** both ESM (index.mjs) and CommonJS (index.js) builds SHALL be available

### Requirement: TypeScript type definitions

The NPM package SHALL include TypeScript type definitions for full IDE support.

#### Scenario: Types available
- **WHEN** TypeScript developer imports the package
- **THEN** type definitions from index.d.ts SHALL be auto-loaded

#### Scenario: TypeScript compilation succeeds
- **WHEN** consumer project runs `tsc --noEmit`
- **THEN** no type errors SHALL occur due to the WASM package types

### Requirement: Shell completion scripts (optional)

The system MAY provide shell completion generation as an npm package capability.

#### Scenario: Completion script export
- **WHEN** consumer calls `getCompletionScript('bash')`
- **THEN** the system SHALL return the bash completion script string