use crate::config::model::McpCatalogue;
use crate::error::AppError;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

/// Synchronises Gemini settings (`.gemini/settings.json`) from the local `.mcp.json` catalogue.
pub struct GeminiSync;

impl GeminiSync {
    pub fn sync(workspace: &Path, catalogue: &McpCatalogue) -> Result<PathBuf, AppError> {
        let gemini_dir = workspace.join(".gemini");
        fs::create_dir_all(&gemini_dir)?;
        let settings_path = gemini_dir.join("settings.json");

        let mut settings = if settings_path.exists() {
            let contents = fs::read_to_string(&settings_path)?;
            serde_json::from_str::<Value>(&contents)?
        } else {
            json!({})
        };

        if let Value::Object(ref mut map) = settings {
            map.insert("mcpServers".to_string(), serde_json::to_value(&catalogue.mcp_servers)?);
        } else {
            return Err(AppError::config(format!(
                "Gemini settings file at {} is not a JSON object",
                settings_path.display()
            )));
        }

        let serialised = serde_json::to_string_pretty(&settings)?;
        fs::write(&settings_path, format!("{serialised}\n"))?;
        Ok(settings_path)
    }
}
