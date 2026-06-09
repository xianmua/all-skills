//! Origin data model

use serde::{Deserialize, Serialize};

/// Represents a git repository origin for skills
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Origin {
    /// Unique name for this origin
    pub name: String,
    /// Git repository URL (HTTPS)
    pub url: String,
    /// Priority (lower number = higher priority)
    pub priority: u32,
    /// Whether this origin is enabled
    pub enabled: bool,
}

impl Origin {
    /// Create a new origin
    #[allow(dead_code)]
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            priority: 100,
            enabled: true,
        }
    }

    /// Create a new origin with custom priority
    #[allow(dead_code)]
    pub fn with_priority(name: String, url: String, priority: u32) -> Self {
        Self {
            name,
            url,
            priority,
            enabled: true,
        }
    }

    /// Validate the URL format
    #[allow(dead_code)]
    pub fn validate_url(&self) -> bool {
        url::Url::parse(&self.url).is_ok()
    }

    /// Get the git clone URL
    #[allow(dead_code)]
    pub fn clone_url(&self) -> String {
        // Remove .git suffix if present
        self.url.trim_end_matches(".git").to_string()
    }
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) - {}", self.name, self.url, if self.enabled { "enabled" } else { "disabled" })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_creation() {
        let origin = Origin::new("github".to_string(), "https://github.com/xianmua/skills".to_string());
        assert_eq!(origin.name, "github");
        assert_eq!(origin.url, "https://github.com/xianmua/skills");
        assert!(origin.enabled);
        assert_eq!(origin.priority, 100);
    }

    #[test]
    fn test_url_validation() {
        let origin = Origin::new("test".to_string(), "https://example.com/repo".to_string());
        assert!(origin.validate_url());

        let invalid = Origin::new("test".to_string(), "not-a-url".to_string());
        assert!(!invalid.validate_url());
    }

    #[test]
    fn test_clone_url() {
        let origin = Origin::new("test".to_string(), "https://github.com/xianmua/skills.git".to_string());
        assert_eq!(origin.clone_url(), "https://github.com/xianmua/skills");
    }
}