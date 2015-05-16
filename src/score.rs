use std::cmp;

pub fn score(haystack: &str, needle: &str) -> u16 {
    if needle.is_empty() || !is_subsequence_of(needle, haystack) {
        return 0
    }

    let mut table = vec![vec![0; haystack.chars().count() + 1];
                         needle.chars().count() + 1];

    for (i, n) in needle.chars().enumerate() {
        for (j, h) in haystack.chars().enumerate().skip(i) {
            table[i+1][j+1] = if n == h {
                table[i][j] + 1
            } else {
                cmp::max(table[i][j+1], table[i+1][j])
            };
        }
    }

    *table.last().unwrap().last().unwrap()
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

    // Match.
    assert_eq!(score("πr²", "π"), 1);
    assert_eq!(score("πr²", "r"), 1);
    assert_eq!(score("πr²", "²"), 1);
    assert_eq!(score("πr²", "r²"), 2);
    assert_eq!(score("πr²", "π²"), 2);
    assert_eq!(score("πr²", "πr²"), 3);
}
