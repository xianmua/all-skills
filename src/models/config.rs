//! Configuration model

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::Origin;

/// Configuration validation error
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// Invalid URL format
    InvalidUrl(String),
    /// Origin name already exists
    OriginAlreadyExists(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidUrl(url) => write!(f, "Invalid URL format: {}", url),
            ConfigError::OriginAlreadyExists(name) => write!(f, "Origin '{}' already exists", name),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Validate that a URL is a valid git repository URL
pub fn validate_git_url(url: &str) -> Result<(), ConfigError> {
    // Support HTTPS URLs
    if url.starts_with("https://") || url.starts_with("http://") {
        if url.contains(' ') {
            return Err(ConfigError::InvalidUrl(url.to_string()));
        }
        return Ok(());
    }

    // Support SSH URLs (git@host:path)
    if url.starts_with("git@") {
        return Ok(());
    }

    // Support local file paths starting with / or ./
    if url.starts_with('/') || url.starts_with("./") {
        return Ok(());
    }

    // Reject any other scheme (anything with :// that's not http/https)
    if url.contains("://") {
        return Err(ConfigError::InvalidUrl(url.to_string()));
    }

    // Accept if it looks like a valid path or simple name
    Ok(())
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Git repository origins
    #[serde(default)]
    pub origins: HashMap<String, OriginConfig>,

    /// Default settings
    #[serde(default)]
    pub defaults: DefaultConfig,

    /// Installed skills metadata
    #[serde(default)]
    pub installed_skills: HashMap<String, InstalledSkill>,
}

/// Configuration for a single origin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginConfig {
    /// Git repository URL
    pub url: String,
    /// Priority (lower = higher priority)
    #[serde(default = "default_priority")]
    pub priority: u32,
    /// Whether this origin is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_priority() -> u32 {
    100
}

fn default_enabled() -> bool {
    true
}

/// Default configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    /// Default IDE type
    #[serde(default = "default_ide")]
    pub ide: String,
}

fn default_ide() -> String {
    "agent".to_string()
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            ide: default_ide(),
        }
    }
}

/// Metadata for an installed skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSkill {
    /// Skill name
    pub name: String,
    /// Version
    pub version: String,
    /// Origin name
    pub origin: String,
    /// Installation path (relative to config dir or absolute)
    pub install_path: PathBuf,
    /// Installation timestamp
    pub installed_at: u64,
    /// Last update timestamp
    pub updated_at: Option<u64>,
}

impl Config {
    /// Get the default config directory path
    pub fn default_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".all-skills")
            .join("config.toml")
    }

    /// Get the default config directory
    #[allow(dead_code)]
    pub fn default_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".all-skills")
    }

    /// Load configuration from file
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Create default config with default origins
    pub fn with_defaults() -> Self {
        let mut config = Self::default();

        // 添加默认的 skills 源
        config.add_origin(
            "gitlab".to_string(),
            "http://gitlab.app.yuchai.com/yc90115142/skills.git".to_string(),
            Some(100),
        );

        config
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Add or update an origin (with validation)
    /// Returns error if URL is invalid or origin name already exists
    pub fn add_origin(&mut self, name: String, url: String, priority: Option<u32>) -> Result<(), ConfigError> {
        // Validate URL format
        validate_git_url(&url)?;

        // Check if origin already exists
        if self.origins.contains_key(&name) {
            return Err(ConfigError::OriginAlreadyExists(name));
        }

        let config = OriginConfig {
            url,
            priority: priority.unwrap_or(100),
            enabled: true,
        };
        self.origins.insert(name, config);
        Ok(())
    }

    /// Add or update an origin (unconditionally, no validation)
    /// Use this when you want to overwrite existing origin
    pub fn set_origin(&mut self, name: String, url: String, priority: Option<u32>) {
        let config = OriginConfig {
            url,
            priority: priority.unwrap_or(100),
            enabled: true,
        };
        self.origins.insert(name, config);
    }

    /// Remove an origin
    pub fn remove_origin(&mut self, name: &str) -> bool {
        self.origins.remove(name).is_some()
    }

    /// Get all enabled origins sorted by priority
    pub fn get_enabled_origins(&self) -> Vec<(String, &OriginConfig)> {
        let mut origins: Vec<_> = self.origins
            .iter()
            .filter(|(_, config)| config.enabled)
            .map(|(name, config)| (name.clone(), config))
            .collect();

        origins.sort_by(|a, b| a.1.priority.cmp(&b.1.priority));
        origins
    }

    /// Add or update an installed skill
    #[allow(dead_code)]
    pub fn add_installed_skill(&mut self, skill: InstalledSkill) {
        self.installed_skills.insert(skill.name.clone(), skill);
    }

    /// Remove an installed skill
    #[allow(dead_code)]
    pub fn remove_installed_skill(&mut self, name: &str) -> bool {
        self.installed_skills.remove(name).is_some()
    }

    /// Get installed skill by name
    #[allow(dead_code)]
    pub fn get_installed_skill(&self, name: &str) -> Option<&InstalledSkill> {
        self.installed_skills.get(name)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            origins: HashMap::new(),
            defaults: DefaultConfig::default(),
            installed_skills: HashMap::new(),
        }
    }
}

impl OriginConfig {
    /// Convert to Origin model
    #[allow(dead_code)]
    pub fn to_origin(&self, name: &str) -> Origin {
        Origin::with_priority(
            name.to_string(),
            self.url.clone(),
            self.priority,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.origins.is_empty());
        assert_eq!(config.defaults.ide, "agent");
    }

    #[test]
    fn test_add_origin() {
        let mut config = Config::default();
        config.add_origin("github".to_string(), "https://github.com/xianmua/skills".to_string(), None).unwrap();

        assert!(config.origins.contains_key("github"));
        let origin = &config.origins["github"];
        assert_eq!(origin.url, "https://github.com/xianmua/skills");
        assert_eq!(origin.priority, 100);
    }

    #[test]
    fn test_add_origin_duplicate_error() {
        let mut config = Config::default();
        config.add_origin("github".to_string(), "https://github.com/xianmua/skills".to_string(), None).unwrap();

        let result = config.add_origin("github".to_string(), "https://github.com/other/skills".to_string(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_enabled_origins_sorted() {
        let mut config = Config::default();
        config.add_origin("low".to_string(), "https://low.example.com".to_string(), Some(200)).unwrap();
        config.add_origin("high".to_string(), "https://high.example.com".to_string(), Some(50)).unwrap();
        config.add_origin("mid".to_string(), "https://mid.example.com".to_string(), Some(100)).unwrap();

        let origins = config.get_enabled_origins();
        assert_eq!(origins.len(), 3);
        assert_eq!(origins[0].0, "high");
        assert_eq!(origins[1].0, "mid");
        assert_eq!(origins[2].0, "low");
    }

    #[test]
    fn test_validate_git_url_https() {
        assert!(validate_git_url("https://github.com/org/repo").is_ok());
        assert!(validate_git_url("http://gitlab.example.com/repo.git").is_ok());
    }

    #[test]
    fn test_validate_git_url_ssh() {
        assert!(validate_git_url("git@github.com:org/repo").is_ok());
    }

    #[test]
    fn test_validate_git_url_local() {
        assert!(validate_git_url("/home/user/repo").is_ok());
        assert!(validate_git_url("./local/repo").is_ok());
    }

    #[test]
    fn test_validate_git_url_invalid() {
        assert!(validate_git_url("not-a-url").is_ok()); // Simple names are accepted
        assert!(validate_git_url("ftp://invalid").is_err());
    }
}