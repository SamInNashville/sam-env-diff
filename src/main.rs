mod bot_help;
mod differ;
mod formatter;
mod masker;
mod parser;

use clap::Parser;
use std::process;

/// Diff your .env files. Secrets stay masked.
#[derive(Parser)]
#[command(name = "sam-env-diff", version = "1.0.0", author = "Sam M.")]
#[command(about = "Diff your .env files. Secrets stay masked.", long_about = None)]
struct Cli {
    /// First .env file (left)
    left: Option<String>,

    /// Second .env file (right / template)
    right: Option<String>,

    /// Include matching keys in output
    #[arg(long)]
    all: bool,

    /// JSON output, no ANSI
    #[arg(long)]
    bot: bool,

    /// Print machine-readable interface spec
    #[arg(long = "bot-help")]
    bot_help: bool,

    /// Show full values (DANGEROUS — opt-in only)
    #[arg(long)]
    reveal: bool,

    /// Write JSON output to file
    #[arg(short = 'o', long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // --bot-help: print spec and exit 0
    if cli.bot_help {
        let spec = bot_help::bot_help_json();
        println!("{}", serde_json::to_string_pretty(&spec).unwrap());
        process::exit(0);
    }

    // Require left and right
    let left_path = match &cli.left {
        Some(p) => p.clone(),
        None => {
            eprintln!("Error: missing required argument <left>");
            eprintln!("Usage: sam-env-diff <left> <right> [flags]");
            process::exit(2);
        }
    };

    let right_path = match &cli.right {
        Some(p) => p.clone(),
        None => {
            eprintln!("Error: missing required argument <right>");
            eprintln!("Usage: sam-env-diff <left> <right> [flags]");
            process::exit(2);
        }
    };

    // Parse files
    let left_map = match parser::parse_env_file(&left_path) {
        Ok(m) => m,
        Err(e) => {
            if cli.bot {
                eprintln!("{}", serde_json::json!({"error": e, "code": 2}));
            } else {
                eprintln!("Error: {}", e);
            }
            process::exit(2);
        }
    };

    let right_map = match parser::parse_env_file(&right_path) {
        Ok(m) => m,
        Err(e) => {
            if cli.bot {
                eprintln!("{}", serde_json::json!({"error": e, "code": 2}));
            } else {
                eprintln!("Error: {}", e);
            }
            process::exit(2);
        }
    };

    // Diff
    let diff = differ::diff_env_maps(&left_map, &right_map);
    let exit_code = if diff.ok { 0 } else { 1 };

    // Output
    if cli.bot || cli.output.is_some() {
        let bot_output = formatter::json::format_json(&diff, &left_path, &right_path, cli.reveal);
        let json_str = serde_json::to_string_pretty(&bot_output).unwrap();

        if cli.bot {
            println!("{}", json_str);
        }

        if let Some(out_path) = &cli.output {
            if let Err(e) = std::fs::write(out_path, &json_str) {
                eprintln!("Error writing to '{}': {}", out_path, e);
                process::exit(2);
            }
            if !cli.bot {
                eprintln!("✓ Written to {}", out_path);
            }
        }
    } else {
        formatter::console::print_console(&diff, &left_path, &right_path, cli.reveal, cli.all);
    }

    process::exit(exit_code);
}
