//! Shared testing utilities for exercising the `mms` CLI.

use assert_cmd::Command;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Testing harness providing an isolated HOME/workspace pair for CLI scenarios.
#[allow(dead_code)]
pub struct TestContext {
    root: TempDir,
    work_dir: PathBuf,
    original_home: Option<OsString>,
}

#[allow(dead_code)]
impl TestContext {
    /// Create a new isolated environment and point `HOME` to it so the CLI uses local storage.
    pub fn new() -> Self {
        let root = TempDir::new().expect("Failed to create temp directory for tests");
        let work_dir = root.path().join("work");
        fs::create_dir_all(&work_dir).expect("Failed to create test work directory");

        let original_home = env::var_os("HOME");
        unsafe {
            env::set_var("HOME", root.path());
        }

        Self { root, work_dir, original_home }
    }

    /// Absolute path to the emulated `$HOME` directory.
    pub fn home(&self) -> &Path {
        self.root.path()
    }

    /// Path to the workspace directory used for CLI invocations.
    pub fn work_dir(&self) -> &Path {
        &self.work_dir
    }

    /// Convenience helper to create additional sibling workspaces (e.g., for linking scenarios).
    pub fn create_workspace(&self, name: &str) -> PathBuf {
        let path = self.home().join(name);
        fs::create_dir_all(&path).expect("Failed to create additional workspace");
        path
    }

    /// Build a command for invoking the compiled `mms` binary within the default workspace.
    pub fn cli(&self) -> Command {
        self.cli_in(self.work_dir())
    }

    /// Build a command for invoking the compiled `mms` binary within a custom directory.
    pub fn cli_in<P: AsRef<Path>>(&self, dir: P) -> Command {
        let mut cmd = Command::cargo_bin("mms").expect("Failed to locate mms binary");
        cmd.current_dir(dir.as_ref()).env("HOME", self.home());
        cmd
    }

    /// Path to the global `~/.mcp.json` file within the test sandbox.
    pub fn global_mcp_path(&self) -> PathBuf {
        self.home().join(".mcp.json")
    }

    /// Path to the CLI master catalogue file within the sandbox.
    pub fn master_catalogue_path(&self) -> PathBuf {
        self.home().join(".config").join("mms").join("master.json")
    }

    /// Path to the local `.mcp.json` under the default workspace.
    pub fn local_mcp_path(&self) -> PathBuf {
        self.work_dir().join(".mcp.json")
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        match &self.original_home {
            Some(value) => unsafe {
                env::set_var("HOME", value);
            },
            None => unsafe {
                env::remove_var("HOME");
            },
        }
    }
}
