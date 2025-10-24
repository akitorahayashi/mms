use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// Representation of the MCP server catalogue shared across scopes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpCatalogue {
    #[serde(rename = "mcpServers", default)]
    pub mcp_servers: BTreeMap<String, McpServer>,
    #[serde(flatten, default)]
    pub extras: Map<String, Value>,
}

impl McpCatalogue {
    pub fn empty() -> Self {
        Self::default()
    }
}

/// Configuration for a single MCP server entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpServer {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(flatten, default)]
    pub extra: Map<String, Value>,
}

impl McpServer {
    /// Compose the launch command for display purposes.
    pub fn render_command(&self) -> Option<String> {
        let base = self.command.as_ref()?.trim();
        if self.args.is_empty() {
            Some(base.to_string())
        } else {
            let args = self
                .args
                .iter()
                .map(|a| {
                    let trimmed = a.trim();
                    if trimmed.contains(char::is_whitespace) {
                        format!("\"{trimmed}\"")
                    } else {
                        trimmed.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            Some(format!("{base} {args}"))
        }
    }
}
