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

        terminal.info(format!("正在来源中搜索 '{}'...", self.keyword));

        if config.origins.is_empty() {
            terminal.warn("未配置任何来源。请使用 'all-skills add-origin <url>' 添加来源。");
            return Ok(());
        }

        let mut results_found = false;

        for (origin_name, origin_config) in config.get_enabled_origins() {
            terminal.info(format!("\n在 '{}' ({}) 中搜索...", origin_name, origin_config.url));

            // 检查仓库是否可访问
            match git_client.remote_file_exists(&origin_config.url, "", "HEAD") {
                Ok(true) => {
                    terminal.success(format!("  来源 '{}' 可访问", origin_name));
                    terminal.info("  (详细 skill 搜索功能待实现)");
                    terminal.info("  请手动使用 'git clone' 浏览可用的 skills");
                    results_found = true;
                }
                Ok(false) => {
                    terminal.warn(format!("  来源 '{}' 不可访问", origin_name));
                }
                Err(e) => {
                    terminal.error(format!("  访问 '{}' 时出错: {}", origin_name, e));
                }
            }
        }

        if !results_found {
            terminal.warn("在所有配置的来源中未找到结果。");
        }

        Ok(())
    }
}
