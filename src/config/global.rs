use super::master::MasterCatalogue;
use super::model::{McpCatalogue, McpServer};
use super::paths::MmsPaths;
use crate::error::AppError;
use std::fs;
use std::path::PathBuf;

/// Manager for the user-wide `~/.mcp.json` catalogue.
pub struct GlobalCatalogue;

impl GlobalCatalogue {
    pub fn ensure(paths: &MmsPaths) -> Result<McpCatalogue, AppError> {
        let path = paths.global_catalogue_path();
        if !path.exists() {
            MasterCatalogue::write_embedded(paths)?;
            Self::write_from_master(paths)?;
        }
        Self::load(paths)
    }

    pub fn load(paths: &MmsPaths) -> Result<McpCatalogue, AppError> {
        Self::read_from_path(paths.global_catalogue_path())
    }

    pub fn save(paths: &MmsPaths, catalogue: &McpCatalogue) -> Result<(), AppError> {
        let serialised = serde_json::to_string_pretty(catalogue)?;
        fs::write(paths.global_catalogue_path(), format!("{serialised}\n"))?;
        Ok(())
    }

    pub fn write_from_master(paths: &MmsPaths) -> Result<(), AppError> {
        let mut master = MasterCatalogue::load(paths)?;
        for server in master.mcp_servers.values_mut() {
            apply_env_substitutions(server);
        }
        let serialised = serde_json::to_string_pretty(&master)?;
        fs::write(paths.global_catalogue_path(), format!("{serialised}\n"))?;
        Ok(())
    }

    fn read_from_path(path: PathBuf) -> Result<McpCatalogue, AppError> {
        if !path.exists() {
            return Err(AppError::missing_file(path));
        }
        let contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }
}

fn apply_env_substitutions(server: &mut McpServer) {
    for value in server.env.values_mut() {
        if let Some(replacement) = expand_env_placeholder(value) {
            *value = replacement;
        }
    }
}

fn expand_env_placeholder(value: &str) -> Option<String> {
    if value.starts_with("${") && value.ends_with('}') && value.len() > 3 {
        let key = &value[2..value.len() - 1];
        std::env::var(key).ok()
    } else {
        None
    }
}
