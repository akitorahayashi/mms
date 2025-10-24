use super::model::McpCatalogue;
use super::paths::MmsPaths;
use crate::error::AppError;
use std::fs;

/// Embedded authoritative catalogue shipped with the CLI.
pub struct MasterCatalogue;

impl MasterCatalogue {
    pub const EMBEDDED_JSON: &'static str = include_str!("master_data.json");

    /// Ensure the master catalogue exists in the CLI config directory and return it.
    pub fn load(paths: &MmsPaths) -> Result<McpCatalogue, AppError> {
        paths.ensure_config_dir()?;
        let master_path = paths.master_catalogue_path();
        if !master_path.exists() {
            Self::write_embedded(paths)?;
        }
        let contents = fs::read_to_string(&master_path)?;
        match serde_json::from_str(&contents) {
            Ok(catalogue) => Ok(catalogue),
            Err(_) => {
                // File might be corrupt, attempt to recover by overwriting.
                Self::write_embedded(paths)?;
                let new_contents = fs::read_to_string(&master_path)?;
                serde_json::from_str(&new_contents).map_err(Into::into)
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn embedded_json_is_valid() {
        // Ensure the embedded JSON can be parsed into McpCatalogue
        let catalogue: McpCatalogue = serde_json::from_str(MasterCatalogue::EMBEDDED_JSON)
            .expect("Embedded master catalogue should be valid JSON");

        assert!(!catalogue.mcp_servers.is_empty(), "Master catalogue should contain servers");
    }

    #[test]
    fn all_servers_have_required_fields() {
        let catalogue: McpCatalogue = serde_json::from_str(MasterCatalogue::EMBEDDED_JSON)
            .expect("Embedded master catalogue should be valid JSON");

        for (name, server) in &catalogue.mcp_servers {
            assert!(server.command.is_some(), "Server '{}' should have a command", name);
            assert!(
                !server.command.as_ref().unwrap().is_empty(),
                "Server '{}' command should not be empty",
                name
            );
            // args can be empty, description is optional
        }
    }

    #[test]
    fn env_placeholders_are_well_formed() {
        let catalogue: McpCatalogue = serde_json::from_str(MasterCatalogue::EMBEDDED_JSON)
            .expect("Embedded master catalogue should be valid JSON");

        for (name, server) in &catalogue.mcp_servers {
            for (env_key, env_value) in &server.env {
                if env_value.starts_with("${") && env_value.ends_with('}') {
                    let var_name = &env_value[2..env_value.len() - 1];
                    assert!(
                        !var_name.is_empty(),
                        "Server '{}' env var '{}' has empty placeholder",
                        name,
                        env_key
                    );
                    assert!(
                        !var_name.contains(char::is_whitespace),
                        "Server '{}' env var '{}' placeholder contains whitespace",
                        name,
                        env_key
                    );
                }
            }
        }
    }

    #[test]
    fn server_names_are_unique() {
        let catalogue: McpCatalogue = serde_json::from_str(MasterCatalogue::EMBEDDED_JSON)
            .expect("Embedded master catalogue should be valid JSON");

        let mut seen_names = HashSet::new();
        for name in catalogue.mcp_servers.keys() {
            assert!(seen_names.insert(name), "Duplicate server name: {}", name);
        }
    }
}
