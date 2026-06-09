//! 卸载命令

use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::config::Config;
use crate::utils::Terminal;

/// 卸载已安装的 skill
#[derive(Args, Debug)]
#[command(
    name = "uninstall",
    about = "卸载已安装的 skill",
    long_about = None
)]
pub struct Uninstall {
    /// 要卸载的 skill 名称
    #[arg(required = true)]
    pub skill_name: String,

    /// 无需确认直接卸载
    #[arg(short, long)]
    pub force: bool,
}

impl Uninstall {
    /// 执行卸载命令
    pub fn execute(&self, _config: &Config) -> Result<()> {
        let terminal = Terminal::new();

        terminal.info(format!("正在卸载 skill: {}", self.skill_name));

        // 在常见位置查找 skill
        let current_dir = std::env::current_dir()?;
        let mut possible_paths = vec![
            current_dir.join(".agent").join("skills").join(&self.skill_name),
            current_dir.join(".trae").join("skills").join(&self.skill_name),
            current_dir.join(".clion").join("skills").join(&self.skill_name),
            current_dir.join(".vscode").join("skills").join(&self.skill_name),
        ];

        // 动态扫描所有 .xxx/skills 目录
        if let Ok(entries) = std::fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with('.') && name != ".agent" && name != ".trae" && name != ".clion" && name != ".vscode" {
                            let skill_dir = path.join("skills").join(&self.skill_name);
                            if skill_dir.exists() {
                                possible_paths.push(skill_dir);
                            }
                        }
                    }
                }
            }
        }

        let mut found_path: Option<PathBuf> = None;

        for path in &possible_paths {
            if path.exists() && path.is_dir() {
                found_path = Some(path.clone());
                break;
            }
        }

        let skill_path = match found_path {
            Some(p) => p,
            None => {
                terminal.error(format!(
                    "Skill '{}' 未安装在当前目录下。",
                    self.skill_name
                ));
                terminal.info("检查的位置:");
                for path in &possible_paths {
                    terminal.info(format!("  - {:?}", path));
                }
                std::process::exit(1);
            }
        };

        // 确认卸载（除非使用 --force）
        if !self.force {
            if !terminal.confirm(&format!(
                "确定要卸载 '{}' 吗？（位置: {:?}）",
                self.skill_name, skill_path
            )) {
                terminal.info("卸载已取消。");
                return Ok(());
            }
        }

        // 删除 skill 目录
        std::fs::remove_dir_all(&skill_path)?;

        terminal.success(format!("成功卸载 '{}'", self.skill_name));
        terminal.info(format!("已从 {:?} 删除", skill_path));

        Ok(())
    }
}
