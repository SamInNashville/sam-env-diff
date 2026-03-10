/// --bot-help: machine-readable interface spec (~200 tokens).
/// A bot reads this once and knows everything.
use serde_json::{json, Value};

pub fn bot_help_json() -> Value {
    json!({
        "tool": "sam-env-diff",
        "version": "1.0.0",
        "usage": "sam-env-diff <left> <right> [flags]",
        "args": {
            "left": "Path to first .env file",
            "right": "Path to second .env file (the \"template\")"
        },
        "flags": {
            "--all": "Include matching keys in output",
            "--bot": "JSON output, no ANSI",
            "--bot-help": "Print this spec",
            "--reveal": "Show full values (DANGEROUS — opt-in only)",
            "-o <file>": "Write JSON output to file"
        },
        "exit_codes": { "0": "match", "1": "differences found", "2": "error" },
        "bot_output": {
            "left": "string — left file path",
            "right": "string — right file path",
            "missing": "array<{key}> — in right, not in left",
            "extra": "array<{key, val}> — in left, not in right (masked)",
            "changed": "array<{key, left, right}> — different values (masked)",
            "match": "number — count of identical keys",
            "ok": "boolean — true if no differences"
        },
        "masking": "Values show last 4 chars only: ****7f3a. Use --reveal to unmask."
    })
}
