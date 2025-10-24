use clap::{Args, Parser, Subcommand};

/// Manage MCP Servers (mms) CLI.
#[derive(Debug, Parser)]
#[command(name = "mms")]
#[command(about = "Manage MCP Servers", long_about = None)]
pub struct Cli {
    /// Enable verbose output for troubleshooting.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialise a `.mcp.json` catalogue in the current directory.
    #[command(visible_aliases = ["ini"])]
    Init {
        /// Copy entries from the global `~/.mcp.json`.
        #[arg(long = "from-global", visible_alias = "from_global")]
        from_global: bool,
    },

    /// List MCP servers available in the global catalogue.
    #[command(visible_aliases = ["ls"])]
    List,

    /// Add servers from the global catalogue into the project-local file.
    #[command(visible_aliases = ["a"])]
    Add {
        /// Server names to add.
        names: Vec<String>,
    },

    /// Remove a server from the project-local catalogue.
    #[command(visible_aliases = ["rm"])]
    Remove {
        /// Server name to remove.
        name: String,
    },

    /// Show the command used to start a server.
    #[command(visible_aliases = ["cmd"])]
    Command {
        /// Server name to inspect.
        name: String,
        /// Copy the command to the clipboard using pbcopy (macOS).
        #[arg(long)]
        copy: bool,
    },

    /// Synchronise local catalogue with Gemini and Codex configurations.
    #[command(visible_aliases = ["s"])]
    Sync {
        /// Skip updating the Codex configuration.
        #[arg(long = "skip-codex")]
        skip_codex: bool,
        /// Skip updating Gemini settings.
        #[arg(long = "skip-gemini")]
        skip_gemini: bool,
    },

    /// Remove generated configuration artifacts.
    #[command(visible_aliases = ["cln"])]
    Clean {
        #[command(flatten)]
        selection: CleanSelection,
    },
}

#[derive(Debug, Clone, Args)]
pub struct CleanSelection {
    /// Remove everything (local, Gemini, Codex, global, master).
    #[arg(long)]
    pub all: bool,
    /// Remove the project `.mcp.json`.
    #[arg(long)]
    pub local: bool,
    /// Remove project `.gemini/settings.json`.
    #[arg(long)]
    pub gemini: bool,
    /// Remove global `~/.codex/config.toml`.
    #[arg(long)]
    pub codex: bool,
    /// Remove the global `~/.mcp.json`.
    #[arg(long)]
    pub global: bool,
    /// Remove the CLI master catalogue copy.
    #[arg(long)]
    pub master: bool,
    /// Show what would be deleted without making changes.
    #[arg(long)]
    pub dry_run: bool,
}

impl CleanSelection {
    pub fn normalised(mut self) -> Self {
        if self.all {
            self.local = true;
            self.gemini = true;
            self.codex = true;
            self.global = true;
            self.master = true;
        }

        if !self.local && !self.gemini && !self.codex && !self.global && !self.master {
            self.local = true;
        }

        self
    }
}
