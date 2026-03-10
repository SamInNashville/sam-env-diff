/// Parses .env files into key-value maps.
/// Handles: comments, empty lines, quoted values (single/double), export prefix,
/// multiline values, inline comments, KEY=value=with=equals, duplicate keys (last wins),
/// Windows line endings (CRLF), UTF-8 BOM.
use std::collections::HashMap;

pub type EnvMap = HashMap<String, String>;

/// Strip surrounding single or double quotes.
fn strip_quotes(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        &value[1..value.len() - 1]
    } else {
        value
    }
}

/// Parse raw .env content into an EnvMap.
/// Duplicate keys: last definition wins.
pub fn parse_env(content: &str) -> EnvMap {
    let mut map = EnvMap::new();

    // Strip UTF-8 BOM if present
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(content);

    // Normalize CRLF -> LF
    let normalized;
    let content = if content.contains('\r') {
        normalized = content.replace("\r\n", "\n").replace('\r', "\n");
        normalized.as_str()
    } else {
        content
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let raw_line = lines[i];
        i += 1;

        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Strip `export ` prefix
        let line = if let Some(stripped) = trimmed.strip_prefix("export ") {
            stripped.trim_start()
        } else {
            trimmed
        };

        let eq_index = match line.find('=') {
            Some(idx) => idx,
            None => continue, // no `=`, skip
        };

        let key = line[..eq_index].trim();
        if key.is_empty() {
            continue;
        }

        let mut value = line[eq_index + 1..].to_string();

        // Handle double-quoted multiline values
        if value.starts_with('"') && !value.ends_with('"') {
            while i < lines.len() {
                let next_line = lines[i];
                i += 1;
                value.push('\n');
                value.push_str(next_line);
                if next_line.ends_with('"') {
                    break;
                }
            }
        }

        // Handle single-quoted multiline values
        if value.starts_with('\'') && !value.ends_with('\'') {
            while i < lines.len() {
                let next_line = lines[i];
                i += 1;
                value.push('\n');
                value.push_str(next_line);
                if next_line.ends_with('\'') {
                    break;
                }
            }
        }

        // Strip inline comments (only outside quotes)
        if !value.starts_with('"') && !value.starts_with('\'') {
            if let Some(comment_idx) = value.find(" #") {
                value = value[..comment_idx].to_string();
            }
        }

        let value = strip_quotes(value.trim()).to_string();
        map.insert(key.to_string(), value);
    }

    map
}

/// Parse a .env file from disk.
pub fn parse_env_file(path: &str) -> Result<EnvMap, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read '{}': {}", path, e))?;
    Ok(parse_env(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_key_value() {
        let m = parse_env("FOO=bar\nBAZ=qux\n");
        assert_eq!(m["FOO"], "bar");
        assert_eq!(m["BAZ"], "qux");
    }

    #[test]
    fn skips_comments_and_blank_lines() {
        let m = parse_env("# comment\n\nFOO=bar\n");
        assert_eq!(m.len(), 1);
        assert_eq!(m["FOO"], "bar");
    }

    #[test]
    fn strips_export_prefix() {
        let m = parse_env("export FOO=bar\n");
        assert_eq!(m["FOO"], "bar");
    }

    #[test]
    fn double_quotes() {
        let m = parse_env(r#"FOO="hello world""#);
        assert_eq!(m["FOO"], "hello world");
    }

    #[test]
    fn single_quotes() {
        let m = parse_env("FOO='hello world'");
        assert_eq!(m["FOO"], "hello world");
    }

    #[test]
    fn multiline_double_quote() {
        let m = parse_env("FOO=\"line1\nline2\"");
        assert_eq!(m["FOO"], "line1\nline2");
    }

    #[test]
    fn multiline_single_quote() {
        let m = parse_env("FOO='line1\nline2'");
        assert_eq!(m["FOO"], "line1\nline2");
    }

    #[test]
    fn inline_comment() {
        let m = parse_env("FOO=bar # this is a comment");
        assert_eq!(m["FOO"], "bar");
    }

    #[test]
    fn key_with_equals_in_value() {
        let m = parse_env("FOO=a=b=c");
        assert_eq!(m["FOO"], "a=b=c");
    }

    #[test]
    fn duplicate_keys_last_wins() {
        let m = parse_env("FOO=first\nFOO=second\n");
        assert_eq!(m["FOO"], "second");
    }

    #[test]
    fn crlf_line_endings() {
        let m = parse_env("FOO=bar\r\nBAZ=qux\r\n");
        assert_eq!(m["FOO"], "bar");
        assert_eq!(m["BAZ"], "qux");
    }

    #[test]
    fn utf8_bom() {
        let content = "\u{FEFF}FOO=bar\n";
        let m = parse_env(content);
        assert_eq!(m["FOO"], "bar");
    }

    #[test]
    fn empty_value() {
        let m = parse_env("FOO=\n");
        assert_eq!(m["FOO"], "");
    }

    #[test]
    fn no_equals_skipped() {
        let m = parse_env("JUSTKEY\nFOO=bar\n");
        assert_eq!(m.len(), 1);
        assert_eq!(m["FOO"], "bar");
    }

    #[test]
    fn empty_key_skipped() {
        let m = parse_env("=value\nFOO=bar\n");
        assert_eq!(m.len(), 1);
    }
}
