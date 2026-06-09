//! 移除来源命令

use anyhow::Result;
use clap::Args;

use crate::config::Config;
use crate::utils::Terminal;

/// 移除已配置的来源
#[derive(Args, Debug)]
#[command(
    name = "remove-origin",
    about = "移除已配置的 git 仓库来源",
    long_about = None
)]
pub struct RemoveOrigin {
    /// 要移除的来源名称
    #[arg(required = true)]
    pub name: String,

    /// 无需确认直接移除
    #[arg(short, long)]
    pub force: bool,
}

impl RemoveOrigin {
    /// 执行移除来源命令
    pub fn execute(&self, config: &Config, config_path: &std::path::Path) -> Result<()> {
        let terminal = Terminal::new();

        // 检查来源是否存在
        if !config.origins.contains_key(&self.name) {
            terminal.error(format!("未找到来源 '{}'。", self.name));
            std::process::exit(1);
        }

        // 确认移除
        if !self.force {
            if !terminal.confirm(&format!("确定要移除来源 '{}' 吗？", self.name)) {
                terminal.info("移除已取消。");
                return Ok(());
            }
        }

        // 移除来源
        let mut config = config.clone();
        config.remove_origin(&self.name);

        // 保存配置
        config.save(config_path)?;

        terminal.success(format!("成功移除来源 '{}'", self.name));

        Ok(())
    }
}
