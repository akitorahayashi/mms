use crate::cli::{CleanSelection, Commands};
use crate::config::{GlobalCatalogue, LocalCatalogue, MmsPaths};
use crate::error::AppError;
use crate::integration::{CodexSync, GeminiSync};
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

pub struct CommandContext {
    pub paths: MmsPaths,
    pub start_dir: PathBuf,
    pub verbose: bool,
}

impl CommandContext {
    fn log(&self, message: &str) {
        if self.verbose {
            println!("{message}");
        }
    }
}

pub fn execute(command: Commands, context: CommandContext) -> Result<(), AppError> {
    match command {
        Commands::Init { from_global } => init(from_global, &context),
        Commands::List => list(&context),
        Commands::Add { names } => add(names, &context),
        Commands::Remove { name } => remove(name, &context),
        Commands::Command { name, copy } => show_command(name, copy, &context),
        Commands::Sync { skip_codex, skip_gemini } => sync(skip_codex, skip_gemini, &context),
        Commands::Clean { selection } => clean(selection.normalised(), &context),
    }
}

fn init(from_global: bool, ctx: &CommandContext) -> Result<(), AppError> {
    let cwd = &ctx.start_dir;
    ctx.log(&format!("Initialising local catalogue in {}", cwd.display()));

    if from_global {
        let global = GlobalCatalogue::ensure(&ctx.paths)?;
        let path = LocalCatalogue::init_from_global(cwd, &global)?;
        println!("✅ Created {path}", path = path.display());
    } else {
        let path = LocalCatalogue::init_empty(cwd)?;
        println!("✅ Created empty {}", path.display());
    }
    Ok(())
}

fn list(ctx: &CommandContext) -> Result<(), AppError> {
    let catalogue = GlobalCatalogue::ensure(&ctx.paths)?;
    if catalogue.mcp_servers.is_empty() {
        println!("No MCP servers found in {}", ctx.paths.global_catalogue_path().display());
        return Ok(());
    }

    println!("Available MCP servers:\n=====================");
    for (name, server) in catalogue.mcp_servers.iter() {
        println!("[{name}]");
        if let Some(command) = server.render_command() {
            println!("{command}");
        } else {
            println!("(no command defined)");
        }
        if let Some(description) = &server.description {
            println!("- {description}");
        }
        println!();
    }
    Ok(())
}

fn add(names: Vec<String>, ctx: &CommandContext) -> Result<(), AppError> {
    let (mut local, local_path) = LocalCatalogue::load(&ctx.start_dir)?;
    let global = GlobalCatalogue::ensure(&ctx.paths)?;
    let mut modified = false;

    for name in names {
        if !global.mcp_servers.contains_key(&name) {
            println!(
                "⚠️  MCP server '{name}' not found in {}",
                ctx.paths.global_catalogue_path().display()
            );
            continue;
        }

        if local.mcp_servers.contains_key(&name) {
            println!("ℹ️  MCP server '{name}' already present in local catalogue");
            continue;
        }

        if let Some(server) = global.mcp_servers.get(&name) {
            local.mcp_servers.insert(name.clone(), server.clone());
            println!("✅ Added '{name}' to {}", local_path.display());
            modified = true;
        }
    }

    if modified {
        LocalCatalogue::save(&local_path, &local)?;
    }

    Ok(())
}

fn remove(name: String, ctx: &CommandContext) -> Result<(), AppError> {
    let (mut local, local_path) = LocalCatalogue::load(&ctx.start_dir)?;
    if local.mcp_servers.remove(&name).is_some() {
        LocalCatalogue::save(&local_path, &local)?;
        println!("🗑️  Removed '{name}' from {}", local_path.display());
    } else {
        println!("⚠️  MCP server '{name}' not found in {}", local_path.display());
    }
    Ok(())
}

fn show_command(name: String, copy: bool, ctx: &CommandContext) -> Result<(), AppError> {
    let catalogue = GlobalCatalogue::ensure(&ctx.paths)?;
    let Some(server) = catalogue.mcp_servers.get(&name) else {
        println!(
            "⚠️  MCP server '{name}' not found in {}",
            ctx.paths.global_catalogue_path().display()
        );
        return Ok(());
    };

    if let Some(command) = server.render_command() {
        println!("Command for '{name}': {command}");
        if copy {
            copy_to_clipboard(&command);
        }
    } else {
        println!("⚠️  Server '{name}' does not define a command");
    }

    Ok(())
}

fn sync(skip_codex: bool, skip_gemini: bool, ctx: &CommandContext) -> Result<(), AppError> {
    let (local, local_path) = LocalCatalogue::load(&ctx.start_dir)?;
    let workspace =
        local_path.parent().map(Path::to_path_buf).unwrap_or_else(|| ctx.start_dir.clone());
    ctx.log(&format!("Using workspace {}", workspace.display()));

    let mut updated = Vec::new();

    if !skip_gemini {
        let settings_path = GeminiSync::sync(&workspace, &local)?;
        println!("✅ Synced Gemini settings at {}", settings_path.display());
        updated.push("Gemini");
    }

    if !skip_codex {
        match CodexSync::sync(&ctx.paths, &local)? {
            Some(path) => {
                println!("✅ Synced Codex MCP block at {}", path.display());
                updated.push("Codex");
            }
            None => {
                println!("ℹ️  Codex configuration not found; skipped MCP sync");
            }
        }
    }

    if updated.is_empty() {
        println!("ℹ️  Nothing to synchronise");
    }

    Ok(())
}

fn clean(selection: CleanSelection, ctx: &CommandContext) -> Result<(), AppError> {
    let mut operations = Vec::new();

    if selection.local
        && let Some(path) = LocalCatalogue::discover(&ctx.start_dir)
    {
        operations.push(("local .mcp.json", path));
    }

    if selection.gemini {
        let root = LocalCatalogue::discover(&ctx.start_dir)
            .and_then(|p| p.parent().map(Path::to_path_buf))
            .unwrap_or_else(|| ctx.start_dir.clone());
        operations.push(("Gemini settings", root.join(".gemini").join("settings.json")));
    }

    if selection.master {
        operations.push(("CLI master catalogue", ctx.paths.master_catalogue_path()));
    }

    if selection.global {
        operations.push(("global ~/.mcp.json", ctx.paths.global_catalogue_path()));
    }

    if operations.is_empty() {
        println!("ℹ️  Nothing selected for cleanup");
        return Ok(());
    }

    for (label, path) in operations {
        if selection.dry_run {
            println!("Dry run: would remove {}", path.display());
            continue;
        }

        if path.exists() {
            std::fs::remove_file(&path)?;
            println!("🧹 Removed {label} ({})", path.display());
        } else {
            println!("ℹ️  {label} not found at {}", path.display());
        }
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn copy_to_clipboard(payload: &str) {
    if ProcessCommand::new("pbcopy")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.take() {
                let mut handle = stdin;
                handle.write_all(payload.as_bytes())?;
            }
            child.wait()?;
            Ok(())
        })
        .is_ok()
    {
        println!("📋 Copied to clipboard");
    } else {
        println!("⚠️  Unable to copy to clipboard (pbcopy not available)");
    }
}

#[cfg(not(target_os = "macos"))]
fn copy_to_clipboard(_payload: &str) {
    println!("⚠️  Copying to clipboard is only supported on macOS.");
}
