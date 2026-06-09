//! Error types for all-skills

use thiserror::Error;
use std::path::PathBuf;

/// Result type alias using our error type
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

/// All possible errors in all-skills
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration file not found or invalid
    #[error("Configuration error: {0}")]
    Config(String),

    /// Git operation failed
    #[error("Git error: {0}")]
    Git(String),

    /// Skill not found in any origin
    #[error("Skill '{0}' not found in any configured origin")]
    SkillNotFound(String),

    /// Skill already installed
    #[error("Skill '{0}' is already installed. Use --force to overwrite")]
    SkillAlreadyInstalled(String),

    /// Skill not installed
    #[error("Skill '{0}' is not installed")]
    SkillNotInstalled(String),

    /// Origin not found in configuration
    #[error("Origin '{0}' not found")]
    OriginNotFound(String),

    /// Origin already exists
    #[error("Origin '{0}' already exists")]
    OriginAlreadyExists(String),

    /// Invalid URL format
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parsing error
    #[error("Failed to parse configuration: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// TOML serialization error
    #[error("Failed to serialize configuration: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    /// Git2 error
    #[error("Git operation failed: {0}")]
    Git2(#[from] git2::Error),

    /// Network request error
    #[error("Network request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// Installation directory error
    #[error("Failed to access installation directory: {0}")]
    InstallDir(PathBuf),

    /// User cancelled operation
    #[error("Operation cancelled by user")]
    Cancelled,

    /// YAML parsing error
    #[error("Failed to parse YAML: {0}")]
   YamlParse(#[from] serde_yaml::Error),
}