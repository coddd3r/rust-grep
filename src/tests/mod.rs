use crate::match_by_char;

#[test]
fn check_optional_char() {
    assert!(match_by_char("dogs", "dogs?", false).0);
    assert!(match_by_char("dogx", "dogs?x", false).0);
    assert!(match_by_char("dog", "dogs?", false).0);
}

#[test]
fn check_optional_group() {
    assert!(match_by_char("a", "[abc]?", false).0);
    assert!(!match_by_char("x", "[abc]?", false).0);
    assert!(match_by_char("x", "[abc]?x", false).0);
    assert!(match_by_char("a", "[abc]?x?", false).0);
}

#[test]
fn cc_optional() {
    assert!(match_by_char("act", "ca?t", false).0);
    assert!(match_by_char("act", "ca?a?t", false).0);
    assert!(match_by_char("act", "a?ca?a?t", false).0);
    assert!(match_by_char("ct", "a?ca?a?t", false).0);
}

#[test]
fn check_optional_type() {
    assert!(match_by_char("act", r"c\d?t", false).0);
    assert!(match_by_char("act", r"c\d?t\d?", false).0);
}

#[test]
fn check_wildcard() {
    assert!(match_by_char("dog", r"d.g", false).0);
    assert!(match_by_char("2", r".[^abc]", false).0);
    assert!(!match_by_char("2", r".[abc]", false).0);
    assert!(match_by_char("2", r".\d", false).0);
    assert!(match_by_char("dog", r"d.g.", false).0);
    assert!(match_by_char("dog", r".d.g.", false).0);
    assert!(!(match_by_char("dog", r".c.g.", false).0));
    assert!(!(match_by_char("cog", r"d.g", false).0));
}

#[test]
fn check_qty_wildcard() {
    assert!(match_by_char("goøö0Ogol", "g.+gol", false).0);
}

#[test]
fn failed_before_tester() {
    //assert!(!match_by_char("sally has 1 dog", r"\d \w\w\ws", false).0);
    assert!(
        match_by_char(
            "I see 1 cat, 2 dogs and 3 cows",
            r"^I see (\d (cat|dog|cow)s?(, | and )?)+$",
            false
        )
        .0
    )
}

#[test]
fn layered_groups() {
    //echo -n "I see 1 cat, 2 dogs and 3 cows" | ./your_program.sh -E "^I see (\d (cat|dog|cow)(, | and )?)+$
    assert!(
        !match_by_char(
            "I see 1 cat, 2 dogs and 3 cows",
            r"^I see (\d (cat|dog|cow)(, | and )?)+$",
            false
        )
        .0
    )
}

#[test]
fn match_multiple_patterns() {
    assert!(match_by_char("cat", "(cat|dog)", false).0);
    assert!(match_by_char("a cat", "a (cat|dog)", false).0);
    assert!(match_by_char("a cat is", "a (cat|dog) is", false).0);
}
