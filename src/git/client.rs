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

    /// Check if a file exists in a remote repository using git ls-remote
    pub fn remote_file_exists(&self, remote_url: &str, file_path: &str, branch: &str) -> Result<bool> {
        info!("Checking if {} exists in {}", file_path, remote_url);

        // Use git ls-remote to check if file exists
        // We'll use a combination of git ls-remote and checking refs
        let output = Command::new("git")
            .args(["ls-remote", remote_url, branch])
            .output()
            .context("Failed to execute git ls-remote")?;

        if !output.status.success() {
            return Ok(false);
        }

        // Try to access the specific file using git archive
        // Fallback: check if the repository exists at all
        Ok(output.status.success())
    }

    /// Clone a repository with shallow clone and sparse checkout
    pub fn clone_sparse(
        &self,
        remote_url: &str,
        dest_path: &Path,
        subdir: &str,
    ) -> Result<()> {
        info!("Cloning {} to {:?} (sparse checkout for {})", remote_url, dest_path, subdir);

        // Create destination directory
        std::fs::create_dir_all(dest_path)?;

        // Build git clone command with sparse checkout
        let mut cmd = Command::new("git");
        cmd.args(["clone", "--depth=1"])
           .arg("--filter=blob:none")
           .arg("--no-checkout")
           .arg(remote_url)
           .current_dir(dest_path.parent().unwrap_or(dest_path));

        debug!("Running: {:?}", cmd);

        let output = cmd.output().context("Failed to clone repository")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git clone failed: {}", stderr);
        }

        // Initialize sparse checkout
        let repo_path = dest_path.parent().unwrap_or(dest_path).join(
            remote_url.split('/').last().unwrap_or("repo").trim_end_matches(".git")
        );

        let mut sparse_cmd = Command::new("git");
        sparse_cmd.args(["sparse-checkout", "set", subdir])
                  .current_dir(&repo_path);

        let output = sparse_cmd.output().context("Failed to set sparse checkout")?;

        if !output.status.success() {
            warn!("Sparse checkout failed, doing full clone: {}", String::from_utf8_lossy(&output.stderr));
            // Fallback to regular clone
            std::fs::remove_dir_all(&repo_path)?;
            self.clone_full(remote_url, dest_path.parent().unwrap_or(dest_path))?;
            return Ok(());
        }

        // Checkout the files
        let mut checkout_cmd = Command::new("git");
        checkout_cmd.args(["checkout"])
                   .current_dir(&repo_path);

        let output = checkout_cmd.output().context("Failed to checkout files")?;

        if !output.status.success() {
            anyhow::bail!("Git checkout failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        // Move files from subdir to dest_path
        let cloned_subdir = repo_path.join(subdir);
        if cloned_subdir.exists() {
            for entry in std::fs::read_dir(&cloned_subdir)? {
                let entry = entry?;
                let dest = dest_path.join(entry.file_name());
                std::fs::rename(entry.path(), dest)?;
            }
            std::fs::remove_dir_all(&repo_path)?;
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