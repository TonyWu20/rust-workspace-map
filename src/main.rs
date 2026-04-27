use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rust-workspace-map",
    version,
    about = "Generate a JSON map of a Rust workspace's public API surface"
)]
struct Cli {
    /// Path to the workspace root or any directory within it
    #[arg(value_name = "PATH")]
    path: PathBuf,

    /// Write JSON output to file instead of stdout
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let workspace_path = std::path::absolute(&cli.path)
        .map_err(|e| anyhow::anyhow!("invalid path {}: {}", cli.path.display(), e))?;

    let config = rust_workspace_map::Config::builder()
        .workspace_path(workspace_path)
        .output_path(cli.output)
        .build();

    rust_workspace_map::run(config)
}
