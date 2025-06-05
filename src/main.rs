// src/main.rs - This is the CLI entry point
// It handles command line arguments and calls the library functions

// Import the clap crate for command line argument parsing
use clap::Parser;
use colored::*; // For colored terminal output
use std::path::PathBuf;

// Import our library functions
use rustbrother::{analyze_directory, generate_report, AnalysisConfig};

// Define the command line interface using clap
// The #[derive(Parser)] macro automatically generates argument parsing code
#[derive(Parser)]
#[command(name = "rustbrother")]
#[command(about = "Hunt down unused CSS in React components")]
#[command(version = "0.1.0")]
struct Cli {
    /// The directory to analyze (e.g., ./src/components)
    #[arg(short, long, value_name = "DIR")]
    path: PathBuf,

    /// Output format: text, json, or html
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Output file (if not specified, prints to stdout)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Include CSS modules in analysis
    #[arg(long, default_value = "true")]
    css_modules: bool,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}

// The main function - this is where the program starts
fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Print what we're doing if verbose mode is on
    if cli.verbose {
        println!("{}", "Starting CSS analysis...".blue().bold());
        println!("Analyzing directory: {}", cli.path.display());
        println!("{}", "Complexity analysis enabled".yellow());
    }

    // Check if the path exists
    if !cli.path.exists() {
        eprintln!("{}", "Error: Directory does not exist".red().bold());
        std::process::exit(1);
    }

    // Create analysis configuration based on CLI arguments
    let config = AnalysisConfig {
        include_css_modules: cli.css_modules,
        include_styled_components: false, // We'll add this later
        ignore_patterns: vec![
            "node_modules".to_string(),
            ".git".to_string(),
            "dist".to_string(),
            "build".to_string(),
        ],
        enable_complexity_warnings: true, // Always enabled
        complexity_threshold: rustbrother::WarningSeverity::Medium, // Always medium
    };

    // Run the analysis using our library
    let result = analyze_directory(&cli.path, &config)?;

    // Generate the report in the requested format
    let report = generate_report(&result, &cli.format)?;

    // Output the report
    match cli.output {
        Some(output_path) => {
            // Write to file
            std::fs::write(&output_path, report)?;
            if cli.verbose {
                println!("{}", format!("Report saved to: {}", output_path.display()).green());
            }
        }
        None => {
            // Print to stdout
            println!("{}", report);
        }
    }

    // Print summary if verbose
    if cli.verbose {
        println!("{}", "Analysis complete!".green().bold());
        println!("Total files scanned: {}", result.total_files_scanned);
        println!("Unused CSS classes found: {}", result.unused_classes.len());
        println!("Complexity warnings found: {}", result.complexity_warnings.len());
    }

    Ok(())
}