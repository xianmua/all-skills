//! Git client for repository operations

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::{debug, info, warn};

/// Git client for performing git operations
pub struct GitClient {
    /// Whether to use shallow clone
    shallow: bool,
}

impl GitClient {
    /// Create a new GitClient
    pub fn new() -> Self {
        Self { shallow: true }
    }

    /// Check if a file exists in a remote repository
    pub fn remote_file_exists(&self, remote_url: &str, file_path: &str, _branch: &str) -> Result<bool> {
        info!("Checking if {} exists in {}", file_path, remote_url);

        // Try to use git archive to check if file exists
        // This works for public repos without cloning
        let output = Command::new("git")
            .args(["archive", "--remote", remote_url, "HEAD", file_path])
            .output();

        match output {
            Ok(o) => {
                if o.status.success() && !o.stdout.is_empty() {
                    return Ok(true);
                }
                // File might not exist, try listing the directory
                info!("git archive failed, trying ls-remote fallback");
            }
            Err(_) => {
                info!("git archive command failed");
            }
        }

        // Fallback: just check if repo is accessible
        let output = Command::new("git")
            .args(["ls-remote", remote_url, "HEAD"])
            .output()
            .context("Failed to execute git ls-remote")?;

        if !output.status.success() {
            return Ok(false);
        }

        // If repo is accessible, we'll try to install and let it fail later if skill doesn't exist
        info!("Repository is accessible, proceeding with install");
        Ok(true)
    }

    /// Clone a repository with shallow clone and sparse checkout
    pub fn clone_sparse(
        &self,
        remote_url: &str,
        dest_path: &Path,
        subdir: &str,
    ) -> Result<()> {
        info!("Cloning {} to {:?} (sparse checkout for {})", remote_url, dest_path, subdir);

        // Create parent directory for the temp clone
        let parent_dir = dest_path.parent().unwrap_or(dest_path);
        std::fs::create_dir_all(parent_dir)?;

        // Extract repo name from URL
        let repo_name = remote_url.split('/').last().unwrap_or("repo").trim_end_matches(".git");
        let temp_repo_path = parent_dir.join(format!("._temp_{}", repo_name));

        // Remove temp dir if exists
        if temp_repo_path.exists() {
            std::fs::remove_dir_all(&temp_repo_path)?;
        }

        // Build git clone command with sparse checkout
        let mut cmd = Command::new("git");
        cmd.args(["clone", "--depth=1", "--filter=blob:none", "--no-checkout"])
           .arg(remote_url)
           .arg(&temp_repo_path);

        debug!("Running: {:?}", cmd);

        let output = cmd.output().context("Failed to clone repository")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git clone failed: {}", stderr);
        }

        // Initialize sparse checkout
        let mut sparse_cmd = Command::new("git");
        sparse_cmd.args(["sparse-checkout", "set", subdir])
                  .current_dir(&temp_repo_path);

        let output = sparse_cmd.output().context("Failed to set sparse checkout")?;

        if !output.status.success() {
            warn!("Sparse checkout failed, doing full clone: {}", String::from_utf8_lossy(&output.stderr));
            // Fallback to regular clone
            std::fs::remove_dir_all(&temp_repo_path)?;
            self.clone_full(remote_url, parent_dir)?;
            return Ok(());
        }

        // Checkout the files
        let mut checkout_cmd = Command::new("git");
        checkout_cmd.args(["checkout"])
                   .current_dir(&temp_repo_path);

        let output = checkout_cmd.output().context("Failed to checkout files")?;

        if !output.status.success() {
            anyhow::bail!("Git checkout failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        // Move files from subdir to dest_path
        let cloned_subdir = temp_repo_path.join(subdir);
        if cloned_subdir.exists() {
            // Create dest_path parent if needed
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            // If dest_path exists, remove it
            if dest_path.exists() {
                std::fs::remove_dir_all(dest_path)?;
            }
            // Move the entire subdir to dest_path
            std::fs::rename(&cloned_subdir, dest_path)?;
            // Clean up temp repo
            std::fs::remove_dir_all(&temp_repo_path)?;
        } else {
            anyhow::bail!("Cloned subdirectory {:?} does not exist", cloned_subdir);
        }

        Ok(())
    }

    /// Clone a repository fully
    pub fn clone_full(&self, remote_url: &str, dest_path: &Path) -> Result<()> {
        info!("Cloning {} to {:?}", remote_url, dest_path);

        let mut cmd = Command::new("git");
        if self.shallow {
            cmd.arg("clone").arg("--depth=1");
        } else {
            cmd.arg("clone");
        }
        cmd.arg(remote_url).arg(dest_path);

        debug!("Running: {:?}", cmd);

        let output = cmd.output().context("Failed to clone repository")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git clone failed: {}", stderr);
        }

        Ok(())
    }

    /// Pull latest changes for a local repository
    pub fn pull(&self, repo_path: &Path) -> Result<()> {
        info!("Pulling latest changes in {:?}", repo_path);

        let output = Command::new("git")
            .args(["pull", "origin", "HEAD"])
            .current_dir(repo_path)
            .output()
            .context("Failed to pull changes")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git pull failed: {}", stderr);
        }

        Ok(())
    }

    /// Get the current commit hash
    pub fn get_current_commit(&self, repo_path: &Path) -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_path)
            .output()
            .context("Failed to get current commit")?;

        if !output.status.success() {
            anyhow::bail!("Git rev-parse failed");
        }

        let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(hash)
    }

    /// Check if a directory is a git repository
    pub fn is_repo(&self, path: &Path) -> bool {
        Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .current_dir(path)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get remote URL for a local repository
    #[allow(dead_code)]
    pub fn get_remote_url(&self, repo_path: &Path) -> Result<String> {
        let output = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .current_dir(repo_path)
            .output()
            .context("Failed to get remote URL")?;

        if !output.status.success() {
            anyhow::bail!("Failed to get remote URL");
        }

        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(url)
    }
}

impl Default for GitClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_is_repo() {
        let client = GitClient::new();
        assert!(client.is_repo(Path::new(".")));
        assert!(!client.is_repo(Path::new("/nonexistent/path")));
    }
}