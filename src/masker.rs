//! Value masking logic.
//! Security-first: mask by default, reveal only on explicit opt-in.

const MASK_PREFIX: &str = "****";
const TAIL_LENGTH: usize = 4;

/// Mask a value, showing only the last 4 characters.
/// Short values (≤4 chars) are fully masked.
///
/// Examples:
///   "sk-abc123def7f3a" → "****7f3a"
///   "yes"              → "****"
///   ""                 → "****"
pub fn mask_value(value: &str) -> String {
    if value.len() <= TAIL_LENGTH {
        MASK_PREFIX.to_string()
    } else {
        format!("{}{}", MASK_PREFIX, &value[value.len() - TAIL_LENGTH..])
    }
}

/// Optionally mask a value based on the reveal flag.
pub fn maybe_mask(value: &str, reveal: bool) -> String {
    if reveal {
        value.to_string()
    } else {
        mask_value(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_value_shows_tail() {
        assert_eq!(mask_value("sk-abc123def7f3a"), "****7f3a");
    }

    #[test]
    fn short_value_fully_masked() {
        assert_eq!(mask_value("yes"), "****");
    }

    #[test]
    fn empty_value_masked() {
        assert_eq!(mask_value(""), "****");
    }

    #[test]
    fn exactly_four_chars_masked() {
        assert_eq!(mask_value("abcd"), "****");
    }

    #[test]
    fn five_chars_shows_one() {
        assert_eq!(mask_value("abcde"), "****bcde");
    }

    #[test]
    fn reveal_returns_full() {
        assert_eq!(maybe_mask("secret", true), "secret");
    }

    #[test]
    fn no_reveal_masks() {
        assert_eq!(maybe_mask("secret", false), "****cret");
    }
}
