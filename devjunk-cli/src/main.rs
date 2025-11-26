//! devjunk CLI - Command-line interface for development junk cleanup

use anyhow::Result;
use clap::{Parser, Subcommand};
use devjunk_core::{
    build_clean_plan, execute_clean, scan, CleanResult, JunkKind, ScanConfig, ScanResult,
};
use std::path::PathBuf;

/// DevJunk - A tool for scanning and cleaning development build/cache directories
#[derive(Parser)]
#[command(name = "devjunk")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan directories for development junk
    Scan {
        /// Paths to scan (defaults to current directory)
        #[arg(default_value = ".")]
        paths: Vec<PathBuf>,

        /// Maximum depth to scan
        #[arg(short, long)]
        max_depth: Option<usize>,

        /// Include hidden directories in scan
        #[arg(long, default_value = "false")]
        include_hidden: bool,

        /// Output in JSON format
        #[arg(long, default_value = "false")]
        json: bool,
    },

    /// Clean (delete) development junk directories
    Clean {
        /// Paths to scan and clean
        #[arg(default_value = ".")]
        paths: Vec<PathBuf>,

        /// Perform a dry run (don't actually delete)
        #[arg(long, default_value = "false")]
        dry_run: bool,

        /// Maximum depth to scan
        #[arg(short, long)]
        max_depth: Option<usize>,

        /// Filter by junk kind (can be specified multiple times)
        #[arg(long)]
        kind: Vec<String>,

        /// Skip confirmation prompt
        #[arg(short = 'y', long, default_value = "false")]
        yes: bool,
    },

    /// List supported junk types
    Types,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            paths,
            max_depth,
            include_hidden,
            json,
        } => {
            let config = build_scan_config(paths, max_depth, include_hidden, &[]);
            let result = scan(&config)?;

            if json {
                print_json_result(&result)?;
            } else {
                print_table_result(&result);
            }
        }

        Commands::Clean {
            paths,
            dry_run,
            max_depth,
            kind,
            yes,
        } => {
            let config = build_scan_config(paths, max_depth, false, &kind);
            let result = scan(&config)?;

            if result.items.is_empty() {
                println!("No junk directories found.");
                return Ok(());
            }

            print_table_result(&result);

            // Build plan with all items selected
            let all_paths: Vec<PathBuf> = result.items.iter().map(|i| i.path.clone()).collect();
            let plan = build_clean_plan(&result, &all_paths, dry_run);

            if !yes && !dry_run {
                println!();
                println!(
                    "⚠️  This will delete {} directories ({}).",
                    plan.count(),
                    format_size(result.total_size_bytes())
                );
                print!("Continue? [y/N] ");
                std::io::Write::flush(&mut std::io::stdout())?;

                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            let clean_result = execute_clean(&plan)?;
            print_clean_result(&clean_result);
        }

        Commands::Types => {
            print_junk_types();
        }
    }

    Ok(())
}

fn build_scan_config(
    paths: Vec<PathBuf>,
    max_depth: Option<usize>,
    include_hidden: bool,
    kind_filters: &[String],
) -> ScanConfig {
    let mut config = ScanConfig::new(paths).with_hidden(include_hidden);

    if let Some(depth) = max_depth {
        config = config.with_max_depth(depth);
    }

    // Filter by kind if specified
    if !kind_filters.is_empty() {
        let patterns: Vec<JunkKind> = JunkKind::all()
            .into_iter()
            .filter(|k| {
                let name = format!("{:?}", k).to_lowercase();
                kind_filters.iter().any(|f| name.contains(&f.to_lowercase()))
            })
            .collect();

        if !patterns.is_empty() {
            config = config.with_patterns(patterns);
        }
    }

    config
}

fn print_table_result(result: &ScanResult) {
    if result.items.is_empty() {
        println!("No junk directories found.");
        return;
    }

    // Header
    println!();
    println!(
        "{:<60} {:<15} {:>12} {:>10}",
        "Path", "Type", "Size", "Files"
    );
    println!("{}", "-".repeat(100));

    // Items
    for item in &result.items {
        let path_str = item.path.display().to_string();
        let truncated_path = if path_str.len() > 58 {
            format!("...{}", &path_str[path_str.len() - 55..])
        } else {
            path_str
        };

        println!(
            "{:<60} {:<15} {:>12} {:>10}",
            truncated_path,
            item.kind.display_name(),
            format_size(item.size_bytes),
            item.file_count
        );
    }

    // Summary
    println!("{}", "-".repeat(100));
    println!(
        "Total: {} directories, {}, {} files",
        result.item_count(),
        format_size(result.total_size_bytes()),
        result.total_file_count()
    );
    println!();
}

fn print_json_result(result: &ScanResult) -> Result<()> {
    let json = serde_json::to_string_pretty(result)?;
    println!("{}", json);
    Ok(())
}

fn print_clean_result(result: &CleanResult) {
    println!();

    if result.was_dry_run {
        println!("🔍 DRY RUN - No files were deleted");
        println!();
    }

    if !result.deleted.is_empty() {
        let action = if result.was_dry_run {
            "Would delete"
        } else {
            "Deleted"
        };

        println!(
            "✅ {}: {} directories ({})",
            action,
            result.deleted_count(),
            format_size(result.bytes_freed)
        );
    }

    if !result.failed.is_empty() {
        println!();
        println!("❌ Failed to delete {} directories:", result.failed_count());
        for (path, error) in &result.failed {
            println!("   {} - {}", path.display(), error);
        }
    }

    println!();
}

fn print_junk_types() {
    println!();
    println!("Supported junk directory types:");
    println!();
    println!("{:<20} {:<30}", "Type", "Patterns");
    println!("{}", "-".repeat(50));

    for kind in JunkKind::all() {
        let patterns = kind.patterns().join(", ");
        println!("{:<20} {:<30}", kind.display_name(), patterns);
    }

    println!();
}

/// Format bytes into human-readable string
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
