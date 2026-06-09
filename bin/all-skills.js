#!/usr/bin/env node
/**
 * all-skills CLI wrapper
 * Cross-platform wrapper for all-skills Rust binary
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const os = require('os');

const packageJson = require('../package.json');

// Detect platform and architecture
function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === 'win32') {
    return 'windows';
  } else if (platform === 'darwin') {
    return 'macos';
  } else if (platform === 'linux') {
    return 'linux';
  }
  return null;
}

// Get the binary path or name
function getBinaryInfo() {
  const platform = getPlatform();

  if (!platform) {
    console.error(`Unsupported platform: ${process.platform}`);
    process.exit(1);
  }

  // Check if binary exists locally (development mode)
  const localBinary = path.join(__dirname, '..', 'target', 'release', 'all-skills.exe');
  if (fs.existsSync(localBinary)) {
    return { path: localBinary, isLocal: true };
  }

  // Check for non-.exe on non-windows
  const localBinaryNoExe = path.join(__dirname, '..', 'target', 'release', 'all-skills');
  if (fs.existsSync(localBinaryNoExe)) {
    return { path: localBinaryNoExe, isLocal: true };
  }

  // For production, you would download from GitHub Releases here
  // Example:
  // const version = packageJson.version;
  // const downloadUrl = `https://github.com/xianmua/all-skills/releases/download/v${version}/all-skills-${platform}.zip`;
  // const binaryPath = path.join(os.tmpdir(), `all-skills-${platform}`);

  console.error('Binary not found. Please build with: cargo build --release');
  console.error('Or download from GitHub Releases');
  process.exit(1);
}

// Spawn the actual binary with all arguments
function main() {
  const { path: binaryPath } = getBinaryInfo();
  const args = process.argv.slice(2);

  const child = spawn(binaryPath, args, {
    stdio: 'inherit',
    env: process.env
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });

  child.on('error', (err) => {
    console.error(`Failed to execute: ${err.message}`);
    process.exit(1);
  });
}

main();