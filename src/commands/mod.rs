//! CLI commands module

mod install;
mod uninstall;
mod update;
mod search;
mod list;
mod origin;
mod completion;

pub use install::Install;
pub use uninstall::Uninstall;
pub use update::Update;
pub use search::Search;
pub use list::List;
pub use origin::{AddOrigin, ListOrigins, RemoveOrigin};
pub use completion::Completion;

use clap::Subcommand;

/// 所有可用的 CLI 命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 从配置的仓库安装 skill
    Install(Install),
    /// 卸载已安装的 skill
    Uninstall(Uninstall),
    /// 更新 skill 到最新版本
    Update(Update),
    /// 在配置的仓库中搜索 skill
    Search(Search),
    /// 列出已安装的 skills
    List(List),
    /// 添加新的 git 仓库来源
    AddOrigin(AddOrigin),
    /// 列出所有配置的仓库来源
    ListOrigins(ListOrigins),
    /// 移除已配置的仓库来源
    RemoveOrigin(RemoveOrigin),
    /// 生成 shell 补全脚本
    Completion(Completion),
}
