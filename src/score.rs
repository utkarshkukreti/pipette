pub fn score(haystack: &str, needle: &str) -> u16 {
    if needle.is_empty() || !is_subsequence_of(needle, haystack) {
        return 0
    }
    unimplemented!()
}

/// Returns true if `smaller` is a subsequence of `larger`, that is, all chars
/// in `smaller` are present in `larger` in the same order.
fn is_subsequence_of(smaller: &str, larger: &str) -> bool {
    let mut larger = larger.chars();

    for s in smaller.chars() {
        loop {
            match larger.next() {
                Some(l) if s == l => break,
                Some(_) => {},
                None => return false
            }
        }
    }

    true
}

#[test]
fn test_score() {
    // Empty needle.
    assert_eq!(score("πr²", ""), 0);

    // Not a subsequence.
    assert_eq!(score("πr²", "p"), 0);
    assert_eq!(score("πr²", "πr³"), 0);
}
