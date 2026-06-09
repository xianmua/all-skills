//! all-skills CLI - 管理 git 仓库 skill 包的命令行工具
//!
//! 该工具允许用户从 GitHub、GitLab、Gitee 等 git 仓库安装、更新和卸载 skill 包。

mod commands;
mod config;
mod error;
mod git;
mod models;
mod utils;

use anyhow::Result;
use clap::Parser;
use std::env;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::Config;

const APP_NAME: &str = "all-skills";

/// CLI 参数
#[derive(Parser, Debug)]
#[command(
    name = APP_NAME,
    about = "管理 git 仓库 skill 包的命令行工具",
    long_about = None,
    version,
    author
)]
pub struct Cli {
    /// 启用详细输出
    #[arg(short, long, global = true)]
    verbose: bool,

    /// 自定义配置文件路径
    #[arg(short, long, global = true)]
    config: Option<String>,

    /// 要执行的子命令
    #[command(subcommand)]
    command: commands::Commands,
}

/// 从命令行参数中提取所有 --xxx 格式的参数
/// 例如：--trae -> xxx = "trae", --my-ide -> xxx = "my-ide"
fn extract_ide_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut ide_args = Vec::new();
    let mut found_install = false;

    for arg in &args {
        // 找到 install 子命令后开始收集 --xxx 参数
        if arg == "install" {
            found_install = true;
            continue;
        }

        // 如果还没到 install，跳过
        if !found_install {
            continue;
        }

        // 遇到另一个子命令，停止收集
        if arg == "uninstall" || arg == "update" || arg == "search" ||
           arg == "list" || arg == "add-origin" || arg == "list-origins" ||
           arg == "remove-origin" || arg == "completion" {
            break;
        }

        // 收集 --xxx 格式的参数
        if arg.starts_with("--") && arg.len() > 2 {
            let name = &arg[2..];
            // 排除已知选项
            if !["dir", "force", "verbose", "config", "help"].contains(&name) {
                ide_args.push(name.to_string());
            }
        }
    }

    ide_args
}

/// 移除所有 --xxx 格式的参数，返回清理后的参数列表
fn clean_args(ide_args: &[String]) -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut cleaned = vec![args[0].clone()]; // 保留程序名
    let mut found_install = false;

    // 已知的命令行选项
    let known_options = &[
        "--help", "-h", "--version", "-V",
        "--verbose", "-v", "--config", "-c",
        "--dir", "-d", "--force", "-f",
        "--all", "--check", "--json",
        "--name", "--priority", "--output", "--shell",
    ];

    for arg in &args[1..] {
        // 找到 install 子命令
        if arg == "install" {
            found_install = true;
            cleaned.push(arg.clone());
            continue;
        }

        // 如果还没到 install，保留所有参数
        if !found_install {
            cleaned.push(arg.clone());
            continue;
        }

        // 如果是已知选项，保留
        if known_options.contains(&arg.as_str()) {
            cleaned.push(arg.clone());
            continue;
        }

        // 如果是自定义的 --xxx IDE 参数，跳过
        if arg.starts_with("--") && arg.len() > 2 {
            let name = &arg[2..];
            if ide_args.contains(&name.to_string()) {
                continue; // 跳过 IDE 参数
            }
        }

        // 其他参数保留
        cleaned.push(arg.clone());
    }

    cleaned
}

fn main() -> Result<()> {
    // 初始化日志
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    // 提取自定义 IDE 参数
    let ide_args = extract_ide_args();

    // 清理参数后解析
    let args = clean_args(&ide_args);
    let cli = Cli::parse_from(&args);

    if cli.verbose {
        tracing::info!("已启用详细输出模式");
    }

    // 如果有自定义 IDE 参数，传递给 install 命令
    let config_path = cli.config.clone().map(std::path::PathBuf::from)
        .unwrap_or_else(Config::default_path);

    let mut config = if config_path.exists() {
        Config::load(&config_path)?
    } else {
        info!("未找到配置文件，正在创建默认配置...");
        Config::with_defaults()
    };

    if !config_path.exists() {
        config.save(&config_path)?;
        info!("默认配置已创建: {:?}", config_path);
    }

    // 执行子命令
    match cli.command {
        commands::Commands::Install(cmd) => {
            // 如果有自定义 IDE 参数，设置到命令中
            let mut merged_cmd = cmd;
            if !ide_args.is_empty() {
                merged_cmd.set_ide(ide_args[0].clone());
            }
            merged_cmd.execute(&config)
        }
        commands::Commands::Uninstall(cmd) => cmd.execute(&config),
        commands::Commands::Update(cmd) => cmd.execute(&config),
        commands::Commands::Search(cmd) => cmd.execute(&config),
        commands::Commands::List(cmd) => cmd.execute(&config),
        commands::Commands::AddOrigin(cmd) => cmd.execute(&config, &config_path),
        commands::Commands::ListOrigins(cmd) => cmd.execute(&config),
        commands::Commands::RemoveOrigin(cmd) => cmd.execute(&config, &config_path),
        commands::Commands::Completion(cmd) => cmd.execute(),
    }
}
