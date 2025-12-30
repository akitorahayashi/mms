use crate::config::model::McpCatalogue;
use crate::error::AppError;
use std::fs;
use std::path::PathBuf;
use toml_edit::{value, Array, DocumentMut, Item, Table};

/// Synchronises MCP server definitions into `~/.codex/config.toml`.
pub struct CodexSync;

impl CodexSync {
    pub fn sync(
        paths: &crate::config::paths::MmsPaths,
        catalogue: &McpCatalogue,
    ) -> Result<Option<PathBuf>, AppError> {
        let codex_config = paths.codex_config_path();
        if !codex_config.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&codex_config)?;
        let mut doc: DocumentMut = contents.parse()?;

        doc.remove("mcp_servers");

        let mut root = Table::new();
        root.set_implicit(false);
        doc["mcp_servers"] = Item::Table(root);

        for (name, server) in &catalogue.mcp_servers {
            let mut server_table = Table::new();
            server_table.set_implicit(false);

            if let Some(server_type) = &server.server_type {
                server_table["type"] = value(server_type.clone());
            }

            if let Some(command) = &server.command {
                server_table["command"] = value(command.clone());
            }

            if !server.args.is_empty() {
                let mut array = Array::new();
                for arg in &server.args {
                    array.push(arg.clone());
                }
                server_table["args"] = Item::Value(array.into());
            }

            if !server.env.is_empty() {
                let mut env_table = Table::new();
                env_table.set_implicit(false);
                for (key, val) in &server.env {
                    env_table[key] = value(val.clone());
                }
                server_table["env"] = Item::Table(env_table);
            }

            if let Some(timeout) = server.timeout {
                server_table["timeout"] = value(timeout as i64);
            }

            doc["mcp_servers"][name] = Item::Table(server_table);
        }

        let serialised = doc.to_string();
        fs::write(&codex_config, serialised)?;
        Ok(Some(codex_config))
    }
}
