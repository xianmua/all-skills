//! Data models for yc-skills

mod skill;
mod origin;
mod config;

pub use origin::Origin;
pub use config::{Config, ConfigError, validate_git_url};