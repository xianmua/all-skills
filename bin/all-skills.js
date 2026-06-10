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

// Get the binary path
function getBinaryInfo() {
  const platform = getPlatform();

  if (!platform) {
    console.error(`Unsupported platform: ${process.platform}`);
    process.exit(1);
  }

  // Check local development binary first
  const localDevBinary = path.join(__dirname, '..', 'target', 'release', 'all-skills');
  if (fs.existsSync(localDevBinary)) {
    return localDevBinary;
  }

  // Check for .exe in development
  const localDevExe = path.join(__dirname, '..', 'target', 'release', 'all-skills.exe');
  if (fs.existsSync(localDevExe)) {
    return localDevExe;
  }

  // Production: use bundled binary
  const binaryName = platform === 'windows' ? 'all-skills.exe' : 'all-skills';
  const bundledBinary = path.join(__dirname, binaryName);

  if (fs.existsSync(bundledBinary)) {
    // Make executable on Unix
    if (platform !== 'windows') {
      fs.chmodSync(bundledBinary, '755');
    }
    return bundledBinary;
  }

  console.error('Binary not found.');
  process.exit(1);
}

// Spawn the actual binary with all arguments
function main() {
  const binaryPath = getBinaryInfo();
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