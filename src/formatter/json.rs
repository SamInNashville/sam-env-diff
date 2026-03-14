use crate::differ::DiffResult;
use crate::masker::maybe_mask;
/// Bot JSON output formatter.
/// Token-optimized: short keys, no nulls, match is a count.
use serde::Serialize;

#[derive(Serialize)]
pub struct BotMissingEntry {
    pub key: String,
}

#[derive(Serialize)]
pub struct BotExtraEntry {
    pub key: String,
    pub val: String,
}

#[derive(Serialize)]
pub struct BotChangedEntry {
    pub key: String,
    pub left: String,
    pub right: String,
}

#[derive(Serialize)]
pub struct BotOutput {
    pub left: String,
    pub right: String,
    pub missing: Vec<BotMissingEntry>,
    pub extra: Vec<BotExtraEntry>,
    pub changed: Vec<BotChangedEntry>,
    #[serde(rename = "match")]
    pub match_count: usize,
    pub ok: bool,
}

/// Format a DiffResult as token-optimized bot JSON.
pub fn format_json(
    diff: &DiffResult,
    left_path: &str,
    right_path: &str,
    reveal: bool,
) -> BotOutput {
    BotOutput {
        left: left_path.to_string(),
        right: right_path.to_string(),
        missing: diff
            .missing
            .iter()
            .map(|e| BotMissingEntry { key: e.key.clone() })
            .collect(),
        extra: diff
            .extra
            .iter()
            .map(|e| BotExtraEntry {
                key: e.key.clone(),
                val: maybe_mask(e.left_val.as_deref().unwrap_or(""), reveal),
            })
            .collect(),
        changed: diff
            .changed
            .iter()
            .map(|e| BotChangedEntry {
                key: e.key.clone(),
                left: maybe_mask(e.left_val.as_deref().unwrap_or(""), reveal),
                right: maybe_mask(e.right_val.as_deref().unwrap_or(""), reveal),
            })
            .collect(),
        match_count: diff.match_entries.len(),
        ok: diff.ok,
    }
}
