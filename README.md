# all-skills

CLI tool for managing skill packages from git repositories.

## Installation

### Via npm (recommended)

```bash
npm install -g skills-new
```

### Via npx (without installation)

```bash
npx skills-new --version
```

## Usage

```bash
# Install a skill
skills install my-skill

# List installed skills
skills list

# Update skills
skills update --all

# Add a git origin
skills add-origin https://github.com/your-org/skills

# Generate shell completion
skills completion bash >> ~/.bashrc
```

## Building from Source

```bash
# Build the Rust binary
cargo build --release

# The binary will be at target/release/skills (or skills.exe on Windows)
```

## Requirements

- Node.js 14+ (for npm wrapper)
- Rust toolchain (for building from source)

## License

MIT