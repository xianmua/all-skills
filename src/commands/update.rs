//! 更新命令

use anyhow::Result;
use clap::Args;

use crate::config::Config;
use crate::git::GitClient;
use crate::utils::Terminal;

/// 更新 skill 到最新版本
#[derive(Args, Debug)]
#[command(
    name = "update",
    about = "更新 skill 到最新版本",
    long_about = None
)]
pub struct Update {
    /// 要更新的 skill 名称（使用 --all 更新所有）
    #[arg(required_unless_present = "all")]
    pub skill_name: Option<String>,

    /// 更新所有已安装的 skills
    #[arg(short, long)]
    pub all: bool,

    /// 检查更新但不下载
    #[arg(short, long)]
    pub check: bool,
}

impl Update {
    /// 执行更新命令
    pub fn execute(&self, config: &Config) -> Result<()> {
        let terminal = Terminal::new();
        let git_client = GitClient::new();

        if self.check {
            return self.check_updates(config, terminal);
        }

        if self.all {
            return self.update_all(config, terminal, &git_client);
        }

        // 更新单个 skill
        let skill_name = self.skill_name.as_ref().unwrap();
        terminal.info(format!("正在更新 skill: {}", skill_name));

        // 查找 skill 安装位置
        let current_dir = std::env::current_dir()?;
        let mut possible_paths = vec![
            current_dir.join(".agent").join("skills").join(skill_name),
            current_dir.join(".trae").join("skills").join(skill_name),
            current_dir.join(".clion").join("skills").join(skill_name),
            current_dir.join(".vscode").join("skills").join(skill_name),
        ];

        // 动态扫描所有 .xxx/skills 目录
        if let Ok(entries) = std::fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with('.') && name != ".agent" && name != ".trae" && name != ".clion" && name != ".vscode" {
                            let skill_dir = path.join("skills").join(skill_name);
                            if skill_dir.exists() {
                                possible_paths.push(skill_dir);
                            }
                        }
                    }
                }
            }
        }

        let mut skill_path: Option<std::path::PathBuf> = None;
        for path in &possible_paths {
            if path.exists() && path.is_dir() {
                skill_path = Some(path.clone());
                break;
            }
        }

        let skill_path = match skill_path {
            Some(p) => p,
            None => {
                terminal.error(format!(
                    "Skill '{}' 未安装在当前目录下。",
                    skill_name
                ));
                std::process::exit(1);
            }
        };

        // 拉取最新更改
        terminal.info("正在拉取最新更改...");

        match git_client.pull(&skill_path) {
            Ok(_) => {
                let new_commit = git_client.get_current_commit(&skill_path)?;
                terminal.success(format!("成功更新 '{}'", skill_name));
                terminal.info(format!("新提交: {}", new_commit));
            }
            Err(e) => {
                terminal.error(format!("更新失败: {}", e));
                terminal.info("该 skill 可能不是 git 仓库或未配置远程仓库。");
                std::process::exit(1);
            }
        }

        Ok(())
    }

    /// 检查更新但不下载
    fn check_updates(&self, _config: &Config, terminal: Terminal) -> Result<()> {
        terminal.info("正在检查更新...");

        if let Some(skill_name) = &self.skill_name {
            terminal.info(format!("正在检查 skill: {}", skill_name));
            terminal.info("(单 skill 更新检查功能待实现)");
        } else {
            terminal.info("(批量更新检查功能待实现)");
        }

        Ok(())
    }

    /// 更新所有已安装的 skills
    fn update_all(&self, _config: &Config, terminal: Terminal, git_client: &GitClient) -> Result<()> {
        terminal.info("正在更新所有已安装的 skills...");

        let current_dir = std::env::current_dir()?;
        let mut found_any = false;

        // 动态扫描所有 .xxx/skills 目录
        if let Ok(entries) = std::fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with('.') {
                            let skills_dir = path.join("skills");
                            if skills_dir.exists() && skills_dir.is_dir() {
                                let ide_name = name.trim_start_matches('.').to_string();
                                found_any = true;

                                terminal.info(format!("\n检查 .{}/skills/...", ide_name));

                                if let Ok(skill_entries) = std::fs::read_dir(&skills_dir) {
                                    for skill_entry in skill_entries.flatten() {
                                        let skill_path = skill_entry.path();
                                        if skill_path.is_dir() {
                                            let skill_name = skill_path.file_name()
                                                .and_then(|n| n.to_str())
                                                .unwrap_or("unknown")
                                                .to_string();

                                            terminal.info(format!("  - {}...", skill_name));

                                            if git_client.is_repo(&skill_path) {
                                                match git_client.pull(&skill_path) {
                                                    Ok(_) => {
                                                        let commit = git_client.get_current_commit(&skill_path)?;
                                                        terminal.success(format!("    已更新到 {}", &commit[..7]));
                                                    }
                                                    Err(_) => {
                                                        terminal.warn(format!("    跳过（不是 git 仓库或拉取失败）"));
                                                    }
                                                }
                                            } else {
                                                terminal.warn(format!("    跳过（不是 git 仓库）"));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if !found_any {
            terminal.info("未找到任何已安装的 skills。");
        }

        terminal.success("\n更新检查完成！");
        Ok(())
    }
}
