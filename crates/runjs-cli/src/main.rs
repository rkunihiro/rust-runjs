use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use runjs_runtime::run_script;
use runjs_runtime::RunOptions;

#[derive(Parser)]
#[command(name = "runjs", version, about = "Run JavaScript/TypeScript scripts with native Rust APIs")]
struct Cli {
    /// Path to the .js or .ts script to run
    script: PathBuf,

    /// Type-check TypeScript before running (not yet implemented in v1)
    #[arg(long)]
    check: bool,

    /// Print extra diagnostic logging
    #[arg(short, long)]
    verbose: bool,

    /// Arguments passed through to the script, available via Native.args()
    #[arg(last = true)]
    script_args: Vec<String>,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    if cli.check {
        eprintln!("error: --check (TypeScript type-checking) is not implemented yet");
        return ExitCode::from(2);
    }

    if !cli.script.exists() {
        eprintln!("error: script not found: {}", cli.script.display());
        return ExitCode::from(2);
    }

    if cli.verbose {
        eprintln!("running {}", cli.script.display());
    }

    let opts = RunOptions {
        script_args: cli.script_args,
    };

    match run_script(&cli.script, opts).await {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(1)
        }
    }
}
