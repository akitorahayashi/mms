//! Integrations that synchronise MCP catalogues with external tooling.

pub mod codex;
pub mod gemini;

pub use codex::CodexSync;
pub use gemini::GeminiSync;
