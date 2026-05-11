mod config;
mod run_tools;
mod server;

use std::path::PathBuf;

pub use config::{config_json, init_agent};
use fabro_client::ServerTarget;
pub use server::start;

#[derive(Debug, Clone)]
pub struct FabroMcpServerSettings {
    pub server_target: Option<ServerTarget>,
    pub storage_dir:   PathBuf,
    pub config_path:   PathBuf,
    pub cwd:           PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct McpConfigSettings {
    pub server:      Option<String>,
    pub storage_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct McpInitSettings {
    pub agent:    McpAgent,
    pub config:   McpConfigSettings,
    pub home_dir: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum McpAgent {
    Claude,
    Cursor,
    Windsurf,
}
