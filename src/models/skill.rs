//! Skill data model

use serde::{Deserialize, Serialize};

/// Represents a skill package
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Name of the skill
    pub name: String,
    /// Version of the skill
    pub version: String,
    /// Description of the skill
    pub description: String,
    /// Origin repository URL where this skill was installed from
    pub origin: String,
    /// Local installation path
    pub install_path: std::path::PathBuf,
    /// IDE type (trae, clion, etc.)
    pub ide: String,
    /// Last update timestamp (Unix timestamp)
    pub updated_at: Option<u64>,
}

impl Skill {
    /// Create a new skill instance
    #[allow(dead_code)]
    pub fn new(
        name: String,
        version: String,
        description: String,
        origin: String,
        install_path: std::path::PathBuf,
        ide: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            origin,
            install_path,
            ide,
            updated_at: None,
        }
    }

    /// Get the manifest file path
    #[allow(dead_code)]
    pub fn manifest_path(&self) -> std::path::PathBuf {
        self.install_path.join("SKILL.md")
    }

    /// Get the manifest.json path (fallback)
    #[allow(dead_code)]
    pub fn manifest_json_path(&self) -> std::path::PathBuf {
        self.install_path.join("manifest.json")
    }

    /// Check if the skill is installed (directory exists)
    #[allow(dead_code)]
    pub fn is_installed(&self) -> bool {
        self.install_path.exists() && self.install_path.is_dir()
    }
}

/// Skill manifest file (SKILL.md or manifest.json)
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    /// Skill metadata
    pub metadata: SkillMetadata,
}

/// Skill metadata from manifest file
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    /// Name of the skill
    pub name: String,
    /// Version string (e.g., "1.0.0")
    pub version: String,
    /// Human-readable description
    pub description: Option<String>,
    /// Author information
    pub author: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// IDE compatibility
    pub ide: Vec<String>,
    /// Required dependencies
    pub dependencies: Option<Vec<String>>,
}

impl Default for SkillMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: "0.1.0".to_string(),
            description: None,
            author: None,
            tags: Vec::new(),
            ide: vec!["trae".to_string()],
            dependencies: None,
        }
    }
}