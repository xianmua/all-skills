//! 搜索命令

use anyhow::Result;
use clap::Args;

use crate::config::Config;
use crate::git::GitClient;
use crate::utils::Terminal;

/// 在配置的仓库中搜索 skill
#[derive(Args, Debug)]
#[command(
    name = "search",
    about = "在配置的 git 仓库中搜索 skill",
    long_about = None
)]
pub struct Search {
    /// 搜索关键词
    #[arg(required = true)]
    pub keyword: String,

    /// 最大显示结果数
    #[arg(short, long, default_value = "20")]
    pub limit: usize,

    /// 显示详细信息
    #[arg(short, long)]
    pub verbose: bool,
}

impl Search {
    /// 执行搜索命令
    pub fn execute(&self, config: &Config) -> Result<()> {
        let terminal = Terminal::new();
        let git_client = GitClient::new();

        if config.origins.is_empty() {
            terminal.warn("未配置任何来源。请使用 'skills add-origin <url>' 添加来源。");
            return Ok(());
        }

        let mut accessible = Vec::new();
        let mut inaccessible = Vec::new();

        for (origin_name, origin_config) in config.get_enabled_origins() {
            match git_client.remote_file_exists(&origin_config.url, "", "HEAD") {
                Ok(true) => {
                    accessible.push((origin_name.clone(), origin_config.url.clone()));
                }
                Ok(false) => {
                    inaccessible.push(origin_name.clone());
                }
                Err(e) => {
                    terminal.error(format!("访问 '{}' 时出错: {}", origin_name, e));
                }
            }
        }

        if accessible.is_empty() && inaccessible.is_empty() {
            terminal.warn("未找到可访问的来源。");
        } else {
            if !accessible.is_empty() {
                terminal.success("可用的 skills 源:");
                for (name, url) in &accessible {
                    println!("  - {}: {}", name, url);
                }
            }
            if !inaccessible.is_empty() {
                terminal.warn(format!("不可访问的源: {}", inaccessible.join(", ")));
            }
        }

        Ok(())
    }
}
