//! 列出命令

use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::config::Config;
use crate::utils::Terminal;

/// 列出已安装的 skills
#[derive(Args, Debug)]
#[command(
    name = "list",
    about = "列出已安装的 skills",
    long_about = None
)]
pub struct List {
    /// 显示详细信息
    #[arg(short, long)]
    pub verbose: bool,

    /// JSON 格式输出
    #[arg(long)]
    pub json: bool,
}

impl List {
    /// 执行列出命令
    pub fn execute(&self, _config: &Config) -> Result<()> {
        let terminal = Terminal::new();
        let current_dir = std::env::current_dir()?;

        let mut skills: Vec<(String, PathBuf, String)> = Vec::new();

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

                                if let Ok(skill_entries) = std::fs::read_dir(&skills_dir) {
                                    for skill_entry in skill_entries.flatten() {
                                        let skill_path = skill_entry.path();
                                        if skill_path.is_dir() {
                                            let skill_name = skill_path.file_name()
                                                .and_then(|n| n.to_str())
                                                .unwrap_or("unknown")
                                                .to_string();

                                            // 检查是否有 skill.yaml、manifest.json 或 SKILL.md
                                            let has_manifest = skill_path.join("skill.yaml").exists()
                                                || skill_path.join("manifest.json").exists()
                                                || skill_path.join("SKILL.md").exists();

                                            if has_manifest {
                                                skills.push((skill_name, skill_path, ide_name.clone()));
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

        if skills.is_empty() {
            terminal.info("当前目录下未安装任何 skills。");
            terminal.info("使用 'all-skills install <name>' 安装 skill。");
            return Ok(());
        }

        if self.json {
            // JSON 输出
            let json_output: Vec<_> = skills.iter()
                .map(|(name, path, ide)| {
                    serde_json::json!({
                        "name": name,
                        "path": path.to_string_lossy(),
                        "ide": ide
                    })
                })
                .collect();

            println!("{}", serde_json::to_string_pretty(&json_output)?);
        } else {
            // 人类可读输出
            terminal.info(format!("\n已安装的 skills（共 {} 个）:\n", skills.len()));

            // 按 IDE 分组
            let mut grouped: HashMap<String, Vec<_>> = HashMap::new();
            for skill in &skills {
                grouped.entry(skill.2.clone()).or_default().push(skill);
            }

            let mut sorted_ides: Vec<_> = grouped.keys().collect();
            sorted_ides.sort();

            for ide in sorted_ides {
                let ide_skills = &grouped[ide];
                terminal.info(format!(".{}:", ide));
                for (name, path, _) in ide_skills {
                    let relative = path.strip_prefix(&current_dir)
                        .unwrap_or(path)
                        .to_string_lossy();

                    if self.verbose {
                        terminal.info(format!("  {} ({})", name, relative));
                    } else {
                        terminal.info(format!("  {}", name));
                    }
                }
                println!();
            }
        }

        Ok(())
    }
}
