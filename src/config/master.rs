use super::model::McpCatalogue;
use super::paths::MmsPaths;
use crate::error::AppError;
use std::fs;

/// Embedded authoritative catalogue shipped with the CLI.
pub struct MasterCatalogue;

impl MasterCatalogue {
    const EMBEDDED_JSON: &'static str = include_str!("master_data.json");

    /// Ensure the master catalogue exists in the CLI config directory and return it.
    pub fn load(paths: &MmsPaths) -> Result<McpCatalogue, AppError> {
        paths.ensure_config_dir()?;
        let master_path = paths.master_catalogue_path();
        if !master_path.exists() {
            Self::write_embedded(paths)?;
        }
        let contents = fs::read_to_string(master_path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Overwrite the stored master catalogue with the embedded version.
    pub fn write_embedded(paths: &MmsPaths) -> Result<(), AppError> {
        let parsed: McpCatalogue = serde_json::from_str(Self::EMBEDDED_JSON)?;
        let serialised = serde_json::to_string_pretty(&parsed)?;
        paths.ensure_config_dir()?;
        fs::write(paths.master_catalogue_path(), format!("{serialised}\n"))?;
        Ok(())
    }
}
