pub fn score(_haystack: &str, needle: &str) -> u16 {
    if needle.is_empty() {
        return 0
    }
    unimplemented!()
}

#[test]
fn test_score() {
    // Empty needle.
    assert_eq!(score("πr²", ""), 0);
}
