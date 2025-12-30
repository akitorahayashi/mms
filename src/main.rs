use clap::Parser;
use mms::cli::Cli;
use mms::config::MmsPaths;
use mms::error::AppError;
use mms::CommandContext;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let paths = MmsPaths::new()?;
    let start_dir = std::env::current_dir()?;
    let context = CommandContext { paths, start_dir, verbose: cli.verbose };
    mms::execute(cli.command, context)
}
