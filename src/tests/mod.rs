use crate::match_by_char;

#[test]
fn check_zero_or_more() {
    assert!(match_by_char("dogs", "dogs?"));
}
