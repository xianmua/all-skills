//! Shell 补全命令

use anyhow::Result;
use clap::{Args, CommandFactory};
use clap_complete::generate;
use std::io;

use crate::utils::Terminal;

/// 生成 shell 补全脚本
#[derive(Args, Debug)]
#[command(
    name = "completion",
    about = "生成 shell 补全脚本",
    long_about = None
)]
pub struct Completion {
    /// 要生成补全脚本的 shell 类型
    #[arg(value_enum, default_value = "bash")]
    pub shell: Shell,

    /// 输出文件（默认为 stdout）
    #[arg(short, long)]
    pub output: Option<std::path::PathBuf>,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Shell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

impl From<Shell> for clap_complete::Shell {
    fn from(shell: Shell) -> Self {
        match shell {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Elvish => clap_complete::Shell::Elvish,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::PowerShell => clap_complete::Shell::PowerShell,
            Shell::Zsh => clap_complete::Shell::Zsh,
        }
    }
}

impl Completion {
    /// 执行补全命令
    pub fn execute(&self) -> Result<()> {
        let terminal = Terminal::new();

        let shell: clap_complete::Shell = self.shell.clone().into();

        // 构建 CLI 用于生成补全
        let mut cmd = crate::Cli::command();

        if let Some(ref output_path) = self.output {
            let file = std::fs::File::create(output_path)?;
            let mut buf = io::BufWriter::new(file);
            generate(shell, &mut cmd, "yc-skills", &mut buf);
            terminal.success(format!(
                "补全脚本已写入 {:?}",
                output_path
            ));
        } else {
            generate(shell, &mut cmd, "yc-skills", &mut io::stdout());
        }

        Ok(())
    }
}
