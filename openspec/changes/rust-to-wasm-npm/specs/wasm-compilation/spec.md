## ADDED Requirements

### Requirement: WASM module compilation

The system SHALL compile the Rust core logic into a WebAssembly (WASM) module using wasm-bindgen and wasm-pack.

#### Scenario: Successful WASM build
- **WHEN** developer runs `wasm-pack build --target nodejs --release`
- **THEN** the system generates a WASM binary compatible with Node.js environments

#### Scenario: Size optimization enabled
- **WHEN** WASM build completes
- **THEN** the resulting binary SHALL be optimized for size (LTO + wasm-opt)

### Requirement: Node.js target support

The generated WASM module SHALL be callable from Node.js using CommonJS and ES Modules.

#### Scenario: CJS import works
- **WHEN** Node.js application uses `require('@scope/all-skills-wasm')`
- **THEN** the WASM module loads successfully and exports expected functions

#### Scenario: ESM import works
- **WHEN** Node.js application uses `import('@scope/all-skills-wasm')`
- **THEN** the WASM module loads successfully and exports expected functions

### Requirement: Browser environment support

The WASM module SHALL also support loading in browser environments (via wasm-pack --target web).

#### Scenario: Browser async initialization
- **WHEN** browser application calls the initialization function
- **THEN** WASM module loads asynchronously and becomes functional