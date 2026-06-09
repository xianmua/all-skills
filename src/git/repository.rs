//! Repository utilities

use std::path::PathBuf;

/// Repository information
#[allow(dead_code)]
#[derive(Debug)]
pub struct Repository {
    /// Path to the repository
    pub path: PathBuf,
    /// Remote URL
    pub remote_url: String,
    /// Current branch
    pub branch: String,
}

#[allow(dead_code)]
impl Repository {
    /// Create a new repository instance
    pub fn new(path: PathBuf, remote_url: String) -> Self {
        Self {
            path,
            remote_url,
            branch: "main".to_string(),
        }
    }

    /// Check if the repository has local modifications
    pub fn has_modifications(&self) -> bool {
        // This would use git status --porcelain
        false // Simplified for now
    }

    /// Get the skill manifest path within this repository
    pub fn skill_manifest_path(&self, skill_name: &str) -> PathBuf {
        self.path.join(skill_name).join("skill.yaml")
    }

    /// Get the skill manifest JSON path within this repository
    pub fn skill_manifest_json_path(&self, skill_name: &str) -> PathBuf {
        self.path.join(skill_name).join("manifest.json")
    }
}