use anyhow::{anyhow, Result};
use clap::Parser;
use regex::Regex;
use std::path::PathBuf;
use walkdir::WalkDir;

mod parser;
mod stats;

use parser::parse_kdig_file;
use stats::display_summary;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the directory containing .txt files with kdig output
    #[arg(short, long)]
    input: PathBuf,

    /// Enable recursive directory traversal
    #[arg(short, long)]
    recursive: bool,

    /// Regex pattern to match filenames (applied to basename only)
    #[arg(short, long)]
    pattern: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Validate that the input path exists and is a directory
    if !cli.input.exists() {
        return Err(anyhow!(
            "Input path does not exist: {}",
            cli.input.display()
        ));
    }

    if !cli.input.is_dir() {
        return Err(anyhow!(
            "Input path is not a directory: {}",
            cli.input.display()
        ));
    }

    // Compile regex pattern if provided
    let pattern_regex = if let Some(ref pattern) = cli.pattern {
        Some(Regex::new(pattern)?)
    } else {
        None
    };

    // Find all .txt files in the directory
    let mut walker = WalkDir::new(&cli.input);

    // Set max depth based on recursive flag
    if !cli.recursive {
        walker = walker.max_depth(1);
    }

    let txt_files: Vec<PathBuf> = walker
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("txt"))
                .unwrap_or(false)
        })
        .filter(|e| {
            // If pattern is provided, match against filename
            if let Some(ref regex) = pattern_regex {
                if let Some(filename) = e.path().file_name().and_then(|n| n.to_str()) {
                    regex.is_match(filename)
                } else {
                    false
                }
            } else {
                true
            }
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    if txt_files.is_empty() {
        let msg = if cli.pattern.is_some() {
            format!(
                "No .txt files matching pattern '{}' found in directory: {}",
                cli.pattern.as_ref().unwrap(),
                cli.input.display()
            )
        } else {
            format!("No .txt files found in directory: {}", cli.input.display())
        };
        return Err(anyhow!(msg));
    }

    println!("Found {} .txt file(s) to analyze...", txt_files.len());

    // Parse each file and collect statistics
    let mut all_stats = Vec::new();
    let mut parsed_count = 0;
    let mut skipped_count = 0;

    for file in &txt_files {
        match parse_kdig_file(file) {
            Ok(Some(stats)) => {
                all_stats.push(stats);
                parsed_count += 1;
            }
            Ok(None) => {
                skipped_count += 1;
            }
            Err(e) => {
                eprintln!("Error parsing {}: {}", file.display(), e);
                skipped_count += 1;
            }
        }
    }

    println!(
        "Successfully parsed {} file(s), skipped {} file(s)\n",
        parsed_count, skipped_count
    );

    if all_stats.is_empty() {
        return Err(anyhow!(
            "No valid kdig output found in any of the .txt files"
        ));
    }

    // Display summary statistics
    display_summary(&all_stats);

    Ok(())
}
