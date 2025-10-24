use crate::error::AppError;
use std::env;
use std::path::{Path, PathBuf};

/// Resolves canonical filesystem locations used by the CLI.
#[derive(Debug, Clone)]
pub struct MmsPaths {
    home: PathBuf,
    config_dir: PathBuf,
}

impl MmsPaths {
    pub fn new() -> Result<Self, AppError> {
        let home = env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| AppError::config("HOME environment variable not set"))?;
        let config_dir = home.join(".config").join("mms");
        Ok(Self { home, config_dir })
    }

    pub fn ensure_config_dir(&self) -> Result<(), AppError> {
        std::fs::create_dir_all(&self.config_dir)?;
        Ok(())
    }

    pub fn home(&self) -> &Path {
        &self.home
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    pub fn master_catalogue_path(&self) -> PathBuf {
        self.config_dir.join("master.json")
    }

    pub fn global_catalogue_path(&self) -> PathBuf {
        self.home.join(".mcp.json")
    }

    pub fn codex_config_path(&self) -> PathBuf {
        self.home.join(".codex").join("config.toml")
    }

    pub fn codex_dir(&self) -> PathBuf {
        self.home.join(".codex")
    }
}
