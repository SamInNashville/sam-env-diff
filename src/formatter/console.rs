/// Rich terminal output formatter.
/// Color-coded, human-readable.
use colored::Colorize;
use crate::differ::{DiffResult, DiffStatus};
use crate::masker::maybe_mask;

pub fn print_console(
    diff: &DiffResult,
    left_path: &str,
    right_path: &str,
    reveal: bool,
    show_all: bool,
) {
    if reveal {
        eprintln!("{}", "⚠️  WARNING: --reveal is set. Full values are visible in your terminal.".red().bold());
    }

    let visible: Vec<_> = diff.entries.iter()
        .filter(|e| e.status != DiffStatus::Match || show_all)
        .collect();

    if visible.is_empty() {
        println!("{}", "✅ Files match perfectly. No differences found.".green().bold());
        return;
    }

    // Column widths
    let key_width = visible.iter().map(|e| e.key.len()).max().unwrap_or(10).max(3);
    let left_header = format!("Left ({})", left_path);
    let right_header = format!("Right ({})", right_path);
    let lw = left_header.len().max(20);
    let rw = right_header.len().max(20);

    // Header
    println!(
        "{:<12}  {:<width$}  {:<lw$}  {:<rw$}",
        "Status".bold(),
        "Key".bold(),
        left_header.bold(),
        right_header.bold(),
        width = key_width,
        lw = lw,
        rw = rw
    );
    println!("{}", "─".repeat(12 + 2 + key_width + 2 + lw + 2 + rw).dimmed());

    for entry in &visible {
        let (label, left_val, right_val) = match entry.status {
            DiffStatus::Missing => {
                let lv = "—".dimmed().to_string();
                let rv = maybe_mask(entry.right_val.as_deref().unwrap_or(""), reveal);
                ("🔴 MISSING".red().to_string(), lv, rv.red().to_string())
            }
            DiffStatus::Extra => {
                let lv = maybe_mask(entry.left_val.as_deref().unwrap_or(""), reveal);
                let rv = "—".dimmed().to_string();
                ("🟡 EXTRA  ".yellow().to_string(), lv.yellow().to_string(), rv)
            }
            DiffStatus::Changed => {
                let lv = maybe_mask(entry.left_val.as_deref().unwrap_or(""), reveal);
                let rv = maybe_mask(entry.right_val.as_deref().unwrap_or(""), reveal);
                ("🔵 CHANGED".blue().to_string(), lv.blue().to_string(), rv.blue().to_string())
            }
            DiffStatus::Match => {
                let lv = maybe_mask(entry.left_val.as_deref().unwrap_or(""), reveal);
                let rv = maybe_mask(entry.right_val.as_deref().unwrap_or(""), reveal);
                ("✅ MATCH  ".green().to_string(), lv.green().to_string(), rv.green().to_string())
            }
        };

        println!(
            "{:<12}  {:<width$}  {:<lw$}  {:<rw$}",
            label,
            entry.key,
            left_val,
            right_val,
            width = key_width,
            lw = lw,
            rw = rw
        );
    }

    print_summary(diff, show_all);
}

fn print_summary(diff: &DiffResult, show_all: bool) {
    let mut parts: Vec<String> = Vec::new();
    if !diff.missing.is_empty() {
        parts.push(format!("{} missing", diff.missing.len()).red().to_string());
    }
    if !diff.extra.is_empty() {
        parts.push(format!("{} extra", diff.extra.len()).yellow().to_string());
    }
    if !diff.changed.is_empty() {
        parts.push(format!("{} changed", diff.changed.len()).blue().to_string());
    }
    if show_all && !diff.match_entries.is_empty() {
        parts.push(format!("{} match", diff.match_entries.len()).green().to_string());
    } else if !show_all && !diff.match_entries.is_empty() {
        parts.push(format!("{} matching (hidden)", diff.match_entries.len()).dimmed().to_string());
    }

    println!("\n{}", parts.join(" · "));

    if diff.ok {
        println!("{}", "\n✅ Files match. No critical differences.".green().bold());
    } else if !diff.missing.is_empty() {
        println!(
            "{}",
            format!("\n⚠️  {} missing key(s) — your app may break without them.", diff.missing.len())
                .red()
                .bold()
        );
    }
}
