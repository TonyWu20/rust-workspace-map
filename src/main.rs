use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
enum Command {
    /// Generate a JSON map of a Rust workspace's public API surface
    Index {
        /// Path to the workspace root or any directory within it
        path: PathBuf,
        /// Write JSON output to file instead of stdout
        #[arg(short = 'o', long = "output", value_name = "FILE")]
        output: Option<PathBuf>,
        /// Run validation checks (orphan files, dead re-exports)
        #[arg(long)]
        validate: bool,
    },
    /// Look up a symbol or file in a previously generated workspace map
    Lookup {
        /// Path to the workspace root or any directory within it
        path: PathBuf,
        /// Look up by symbol name
        #[arg(long, conflicts_with = "file")]
        symbol: Option<String>,
        /// Look up by file path
        #[arg(long, conflicts_with = "symbol")]
        file: Option<String>,
    },
}

#[derive(Parser)]
#[command(
    name = "rust-workspace-map",
    version,
    about = "Generate a JSON map of a Rust workspace's public API surface"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Index { path, output, validate } => {
            let workspace_path = std::path::absolute(&path)
                .unwrap_or_else(|e| {
                    eprintln!("invalid path {}: {}", path.display(), e);
                    std::process::exit(1);
                });

            let config = if let Some(ref output) = output {
                rust_workspace_map::Config::builder()
                    .workspace_path(workspace_path)
                    .output_path(output.clone())
                    .validate(validate)
                    .build()
            } else {
                rust_workspace_map::Config::builder()
                    .workspace_path(workspace_path)
                    .validate(validate)
                    .build()
            };

            match rust_workspace_map::build_map(&config) {
                Ok(map) => {
                    let exit_code = if validate {
                        map.errors.iter().any(|e| {
                            matches!(
                                e.kind,
                                rust_workspace_map::schema::DiagnosticKind::OrphanFile
                                    | rust_workspace_map::schema::DiagnosticKind::DeadReExport
                            ) && e.severity == rust_workspace_map::schema::ErrorSeverity::Warning
                        })
                    } else {
                        false
                    };

                    if let Some(ref output_path) = config.output_path {
                        let file = match std::fs::File::create(output_path) {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("failed to create output file: {e}");
                                std::process::exit(1);
                            }
                        };
                        let writer = std::io::BufWriter::new(file);
                        if let Err(e) = rust_workspace_map::render::render_to_writer(&map, writer) {
                            eprintln!("error writing JSON to file: {e}");
                            std::process::exit(1);
                        }
                    } else {
                        let stdout = std::io::stdout();
                        if let Err(e) = rust_workspace_map::render::render_to_writer(&map, stdout.lock()) {
                            eprintln!("error writing JSON to stdout: {e}");
                            std::process::exit(1);
                        }
                    }

                    if exit_code {
                        std::process::exit(2);
                    }
                }
                Err(e) => {
                    eprintln!("error: {e:#}");
                    std::process::exit(1);
                }
            }
        }
        Command::Lookup { path, symbol, file } => {
            let workspace_path = std::path::absolute(&path)
                .unwrap_or_else(|e| {
                    eprintln!("invalid path {}: {}", path.display(), e);
                    std::process::exit(1);
                });

            let config = rust_workspace_map::Config::builder()
                .workspace_path(workspace_path)
                .build();

            match rust_workspace_map::build_map(&config) {
                Ok(map) => {
                    match (symbol, file) {
                        (Some(sym), None) => {
                            let result = rust_workspace_map::lookup::lookup_symbol(&map, &sym);
                            let json = match serde_json::to_string_pretty(&result) {
                                Ok(j) => j,
                                Err(_) => {
                                    eprintln!("serialization error");
                                    std::process::exit(1);
                                }
                            };
                            println!("{json}");
                            // Exit 1 on NotFound, 0 otherwise.
                            if matches!(result, rust_workspace_map::lookup::SymbolLookupResult::NotFound) {
                                std::process::exit(1);
                            }
                        }
                        (None, Some(f)) => {
                            match rust_workspace_map::lookup::lookup_file(&map, &f) {
                                Some(result) => {
                                    let json = match serde_json::to_string_pretty(&result) {
                                        Ok(j) => j,
                                        Err(_) => {
                                            eprintln!("serialization error");
                                            std::process::exit(1);
                                        }
                                    };
                                    println!("{json}");
                                }
                                None => {
                                    eprintln!("file not found: {f}");
                                    std::process::exit(1);
                                }
                            }
                        }
                        (Some(_), Some(_)) => {
                            // clap's conflicts_with handles this, but be defensive.
                            eprintln!("cannot specify both --symbol and --file");
                            std::process::exit(1);
                        }
                        (None, None) => {
                            eprintln!("must specify either --symbol or --file");
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("error: {e:#}");
                    std::process::exit(1);
                }
            }
        }
    }
}
