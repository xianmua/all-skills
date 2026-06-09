//! 列出来源命令

use anyhow::Result;
use clap::Args;

use crate::config::Config;
use crate::utils::Terminal;

/// 列出所有配置的来源
#[derive(Args, Debug)]
#[command(
    name = "list-origins",
    about = "列出所有配置的 git 仓库来源",
    long_about = None
)]
pub struct ListOrigins {
    /// 显示详细信息
    #[arg(short, long)]
    pub verbose: bool,

    /// JSON 格式输出
    #[arg(long)]
    pub json: bool,
}

impl ListOrigins {
    /// 执行列出来源命令
    pub fn execute(&self, config: &Config) -> Result<()> {
        let terminal = Terminal::new();

        if config.origins.is_empty() {
            terminal.info("未配置任何来源。");
            terminal.info("使用 'all-skills add-origin <url>' 添加 git 仓库。");
            return Ok(());
        }

        if self.json {
            let json_output: Vec<_> = config.origins.iter()
                .map(|(name, cfg)| {
                    serde_json::json!({
                        "name": name,
                        "url": cfg.url,
                        "priority": cfg.priority,
                        "enabled": cfg.enabled
                    })
                })
                .collect();

            println!("{}", serde_json::to_string_pretty(&json_output)?);
        } else {
            terminal.info(format!("\n已配置的来源（共 {} 个）:\n", config.origins.len()));

            let mut origins: Vec<_> = config.origins.iter().collect();
            origins.sort_by(|a, b| a.1.priority.cmp(&b.1.priority));

            for (name, cfg) in origins {
                let status = if cfg.enabled { "已启用" } else { "已禁用" };

                terminal.info(format!("  {}", name));
                if self.verbose {
                    println!("    URL: {}", cfg.url);
                    println!("    优先级: {}", cfg.priority);
                    println!("    状态: {}", status);
                    println!();
                } else {
                    println!("      {} - {} [优先级: {}]",
                        cfg.url,
                        status,
                        cfg.priority
                    );
                }
            }
        }

        Ok(())
    }
}
