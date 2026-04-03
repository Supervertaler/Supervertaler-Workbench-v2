use strsim::normalized_levenshtein;

/// Calculate the fuzzy match percentage between two strings.
/// Returns a value between 0.0 and 100.0.
pub fn match_percentage(source: &str, candidate: &str) -> f32 {
    (normalized_levenshtein(source, candidate) * 100.0) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!((match_percentage("hello", "hello") - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_no_match() {
        assert!(match_percentage("abc", "xyz") < 50.0);
    }

    #[test]
    fn test_partial_match() {
        let pct = match_percentage("hello world", "hello worlds");
        assert!(pct > 80.0 && pct < 100.0);
    }
}
