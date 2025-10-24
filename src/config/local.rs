use super::model::McpCatalogue;
use crate::error::AppError;
use std::fs;
use std::path::{Path, PathBuf};

/// Helpers for project-local `.mcp.json` files.
pub struct LocalCatalogue;

impl LocalCatalogue {
    pub fn init_empty(target_dir: &Path) -> Result<PathBuf, AppError> {
        let path = target_dir.join(".mcp.json");
        if path.exists() {
            return Err(AppError::config(format!(
                ".mcp.json already exists at {}",
                path.display()
            )));
        }
        let mut catalogue = McpCatalogue::empty();
        catalogue.extras = serde_json::Map::new();
        let serialised = serde_json::to_string_pretty(&catalogue)?;
        fs::write(&path, format!("{serialised}\n"))?;
        Ok(path)
    }

    pub fn init_from_global(target_dir: &Path, global: &McpCatalogue) -> Result<PathBuf, AppError> {
        let path = target_dir.join(".mcp.json");
        if path.exists() {
            return Err(AppError::config(format!(
                ".mcp.json already exists at {}",
                path.display()
            )));
        }
        let serialised = serde_json::to_string_pretty(global)?;
        fs::write(&path, format!("{serialised}\n"))?;
        Ok(path)
    }

    pub fn discover(start_dir: &Path) -> Option<PathBuf> {
        for candidate in start_dir.ancestors() {
            let path = candidate.join(".mcp.json");
            if path.exists() {
                return Some(path);
            }
        }
        None
    }

    pub fn load(start_dir: &Path) -> Result<(McpCatalogue, PathBuf), AppError> {
        if let Some(path) = Self::discover(start_dir) {
            let contents = fs::read_to_string(&path)?;
            let catalogue: McpCatalogue = serde_json::from_str(&contents)?;
            Ok((catalogue, path))
        } else {
            Err(AppError::config("No .mcp.json found in current directory or any parent"))
        }
    }

    pub fn save(path: &Path, catalogue: &McpCatalogue) -> Result<(), AppError> {
        let serialised = serde_json::to_string_pretty(catalogue)?;
        fs::write(path, format!("{serialised}\n"))?;
        Ok(())
    }

    pub fn remove_file(path: &Path) -> Result<bool, AppError> {
        if path.exists() {
            fs::remove_file(path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
