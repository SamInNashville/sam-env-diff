/// Diff engine. Compares two EnvMaps and produces a structured DiffResult.
use crate::parser::EnvMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DiffStatus {
    Missing,
    Extra,
    Changed,
    Match,
}

#[derive(Debug, Clone)]
pub struct DiffEntry {
    pub key: String,
    pub status: DiffStatus,
    pub left_val: Option<String>,
    pub right_val: Option<String>,
}

#[derive(Debug)]
pub struct DiffResult {
    pub entries: Vec<DiffEntry>,
    pub missing: Vec<DiffEntry>,
    pub extra: Vec<DiffEntry>,
    pub changed: Vec<DiffEntry>,
    pub match_entries: Vec<DiffEntry>,
    pub ok: bool,
}

/// Diff two env maps.
/// - missing: in right but not left
/// - extra:   in left but not right
/// - changed: in both but different values
/// - match:   identical
pub fn diff_env_maps(left: &EnvMap, right: &EnvMap) -> DiffResult {
    let mut all_keys: Vec<String> = left.keys().chain(right.keys()).cloned().collect();
    all_keys.sort();
    all_keys.dedup();

    let mut entries: Vec<DiffEntry> = Vec::new();

    for key in all_keys {
        let left_val = left.get(&key).cloned();
        let right_val = right.get(&key).cloned();

        let status = match (&left_val, &right_val) {
            (None, Some(_)) => DiffStatus::Missing,
            (Some(_), None) => DiffStatus::Extra,
            (Some(l), Some(r)) if l != r => DiffStatus::Changed,
            _ => DiffStatus::Match,
        };

        entries.push(DiffEntry {
            key,
            status,
            left_val,
            right_val,
        });
    }

    let missing: Vec<DiffEntry> = entries.iter().filter(|e| e.status == DiffStatus::Missing).cloned().collect();
    let extra: Vec<DiffEntry> = entries.iter().filter(|e| e.status == DiffStatus::Extra).cloned().collect();
    let changed: Vec<DiffEntry> = entries.iter().filter(|e| e.status == DiffStatus::Changed).cloned().collect();
    let match_entries: Vec<DiffEntry> = entries.iter().filter(|e| e.status == DiffStatus::Match).cloned().collect();

    let ok = missing.is_empty() && extra.is_empty() && changed.is_empty();

    DiffResult {
        entries,
        missing,
        extra,
        changed,
        match_entries,
        ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn map(pairs: &[(&str, &str)]) -> EnvMap {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<HashMap<_, _>>()
    }

    #[test]
    fn identical_maps() {
        let left = map(&[("FOO", "bar"), ("BAZ", "qux")]);
        let right = left.clone();
        let diff = diff_env_maps(&left, &right);
        assert!(diff.ok);
        assert_eq!(diff.match_entries.len(), 2);
        assert!(diff.missing.is_empty());
        assert!(diff.extra.is_empty());
        assert!(diff.changed.is_empty());
    }

    #[test]
    fn missing_key() {
        let left = map(&[("FOO", "bar")]);
        let right = map(&[("FOO", "bar"), ("SECRET", "xyz")]);
        let diff = diff_env_maps(&left, &right);
        assert!(!diff.ok);
        assert_eq!(diff.missing.len(), 1);
        assert_eq!(diff.missing[0].key, "SECRET");
    }

    #[test]
    fn extra_key() {
        let left = map(&[("FOO", "bar"), ("EXTRA", "val")]);
        let right = map(&[("FOO", "bar")]);
        let diff = diff_env_maps(&left, &right);
        assert!(!diff.ok);
        assert_eq!(diff.extra.len(), 1);
        assert_eq!(diff.extra[0].key, "EXTRA");
    }

    #[test]
    fn changed_key() {
        let left = map(&[("FOO", "old")]);
        let right = map(&[("FOO", "new")]);
        let diff = diff_env_maps(&left, &right);
        assert!(!diff.ok);
        assert_eq!(diff.changed.len(), 1);
        assert_eq!(diff.changed[0].key, "FOO");
    }

    #[test]
    fn entries_sorted() {
        let left = map(&[("Z", "1"), ("A", "2"), ("M", "3")]);
        let right = left.clone();
        let diff = diff_env_maps(&left, &right);
        let keys: Vec<_> = diff.entries.iter().map(|e| e.key.as_str()).collect();
        assert_eq!(keys, vec!["A", "M", "Z"]);
    }
}
