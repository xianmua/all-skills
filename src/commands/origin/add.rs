//! 添加来源命令

use anyhow::Result;
use clap::Args;

use crate::config::Config;
use crate::models::{validate_git_url, ConfigError};
use crate::utils::Terminal;

/// 添加新的 git 仓库来源
#[derive(Args, Debug)]
#[command(
    name = "add-origin",
    about = "添加新的 git 仓库作为 skill 来源",
    long_about = None
)]
pub struct AddOrigin {
    /// Git 仓库 URL (HTTPS)
    #[arg(required = true)]
    pub url: String,

    /// 自定义来源名称
    #[arg(short, long)]
    pub name: Option<String>,

    /// 优先级（数字越小优先级越高）
    #[arg(short, long)]
    pub priority: Option<u32>,
}

impl AddOrigin {
    /// 执行添加来源命令
    pub fn execute(&self, config: &Config, config_path: &std::path::Path) -> Result<()> {
        let terminal = Terminal::new();

        // 验证 URL 格式
        if let Err(ConfigError::InvalidUrl(_)) = validate_git_url(&self.url) {
            terminal.error(format!("无效的 URL: {}", self.url));
            terminal.info("支持的格式: https://..., git@..., 本地路径");
            std::process::exit(1);
        }

        // 确定来源名称
        let name = self.name.clone().unwrap_or_else(|| {
            // 从 URL 中提取名称
            let url_path = self.url.trim_end_matches(".git");
            url_path.split('/').last().unwrap_or("origin").to_string()
        });

        // 添加来源（带验证）
        let mut config = config.clone();
        if let Err(ConfigError::OriginAlreadyExists(_)) = config.add_origin(name.clone(), self.url.clone(), self.priority) {
            terminal.error(format!("来源 '{}' 已存在。请使用其他名称或先移除它。", name));
            std::process::exit(1);
        }

        // 保存配置
        config.save(config_path)?;

        terminal.success(format!("成功添加来源 '{}'", name));
        terminal.info(format!("  URL: {}", self.url));
        terminal.info(format!("  名称: {}", name));
        if let Some(p) = self.priority {
            terminal.info(format!("  优先级: {}", p));
        }

        Ok(())
    }
}
