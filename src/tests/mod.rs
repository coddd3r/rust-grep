use crate::match_by_char;

#[test]
fn check_optional_char() {
    assert!(match_by_char("dogs", "dogs?"));
    assert!(match_by_char("dogx", "dogs?x"));
    assert!(match_by_char("dog", "dogs?"));
}

#[test]
fn check_optional_group() {
    assert!(match_by_char("a", "[abc]?"));
    assert!(match_by_char("x", "[abc]?"));
    assert!(match_by_char("x", "[abc]?x"));
    assert!(match_by_char("a", "[abc]?x?"));
}

#[test]
fn cc_optional() {
    assert!(match_by_char("act", "ca?t"));
    assert!(match_by_char("act", "ca?a?t"));
    assert!(match_by_char("act", "a?ca?a?t"));
    assert!(match_by_char("ct", "a?ca?a?t"));
}
