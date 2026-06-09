//! 安装命令

use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use tracing::warn;

use crate::config::Config;
use crate::git::GitClient;
use crate::utils::Terminal;

/// 从配置的 git 仓库安装 skill
#[derive(Args, Debug)]
#[command(
    name = "install",
    about = "从配置的 git 仓库安装 skill",
    long_about = None,
    hide_possible_values = true
)]
pub struct Install {
    /// 要安装的 skill 名称
    #[arg(required = true)]
    pub skill_name: String,

    /// IDE 目录（由 main.rs 动态设置，不对外暴露）
    #[arg(skip)]
    pub ide: Option<String>,

    /// 自定义安装目录
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// 强制安装，即使 skill 已存在
    #[arg(short, long)]
    pub force: bool,
}

impl Install {
    /// 设置 IDE 参数（供 main.rs 调用）
    pub fn set_ide(&mut self, ide: String) {
        self.ide = Some(ide);
    }

    /// 执行安装命令
    pub fn execute(&self, config: &Config) -> Result<()> {
        let terminal = Terminal::new();

        terminal.info(format!("正在安装 skill: {}", self.skill_name));

        // 确定安装目录
        let install_dir = self.get_install_dir()?;
        let skill_path = install_dir.join(&self.skill_name);

        // 检查是否已安装
        if skill_path.exists() && !self.force {
            if !terminal.confirm(&format!(
                "Skill '{}' 已安装在 {:?}。是否覆盖?",
                self.skill_name, skill_path
            )) {
                terminal.info("安装已取消。");
                return Ok(());
            }
        }

        // 在配置的 origins 中搜索 skill
        let git_client = GitClient::new();
        let mut found = false;

        for (origin_name, origin_config) in config.get_enabled_origins() {
            terminal.info(format!("正在来源 '{}' 中搜索...", origin_name));

            let url = &origin_config.url;

            // 尝试查找 skill
            if let Ok(true) = git_client.remote_file_exists(
                url,
                &format!("{}/skill.yaml", self.skill_name),
                "HEAD",
            ) {
                terminal.success(format!("在来源 '{}' 中找到 '{}'", origin_name, self.skill_name));

                // 克隆 skill
                terminal.info(format!("正在从 {} 下载...", url));

                // 创建安装目录
                std::fs::create_dir_all(&install_dir)?;

                // 使用 sparse checkout 克隆仓库
                match git_client.clone_sparse(url, &skill_path, &self.skill_name) {
                    Ok(_) => {
                        terminal.success(format!("成功安装 '{}'", self.skill_name));
                        found = true;
                        break;
                    }
                    Err(e) => {
                        warn!("从 {} 克隆失败: {}", url, e);
                        // 降级为完整克隆
                        if git_client.clone_full(url, &skill_path).is_ok() {
                            terminal.success(format!("成功安装 '{}'", self.skill_name));
                            found = true;
                            break;
                        }
                    }
                }
            }
        }

        if !found {
            terminal.error(format!(
                "在所有配置的来源中都未找到 skill '{}'。\
                请使用 'all-skills add-origin <url>' 添加来源",
                self.skill_name
            ));
            std::process::exit(1);
        }

        Ok(())
    }

    /// 根据参数获取安装目录
    fn get_install_dir(&self) -> Result<PathBuf> {
        let current_dir = std::env::current_dir()?;

        if let Some(ref dir) = self.dir {
            return Ok(dir.clone());
        }

        // 如果有 IDE 参数，使用它
        if let Some(ref ide) = self.ide {
            return Ok(current_dir.join(format!(".{}", ide)).join("skills"));
        }

        // 默认 agent
        Ok(current_dir.join(".agent").join("skills"))
    }
}
