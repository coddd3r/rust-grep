use crate::match_by_char;

#[test]
fn check_optional_char() {
    assert!(match_by_char("dogs", "dogs?", false, &Vec::new()).0);
    assert!(match_by_char("dogx", "dogs?x", false, &Vec::new()).0);
    assert!(match_by_char("dog", "dogs?", false, &Vec::new()).0);
}

#[test]
fn check_optional_group() {
    assert!(match_by_char("a", "[abc]", false, &Vec::new()).0);
    assert!(match_by_char("a", "[abc]?", false, &Vec::new()).0);
    assert!(match_by_char("x", "[abc]?", false, &Vec::new()).0);
    assert!(match_by_char("x", "[abc]?x", false, &Vec::new()).0);
    assert!(match_by_char("a", "[abc]?x?", false, &Vec::new()).0);
}

#[test]
fn check_negative_group() {
    assert!(match_by_char("apple", "[^xyz]", false, &Vec::new()).0);
    assert!(!match_by_char("xyz", r"([^xyz]+)", false, &Vec::new()).0);
}

#[test]
fn cc_optional() {
    assert!(match_by_char("act", "ca?t", false, &Vec::new()).0);
    assert!(match_by_char("act", "ca?a?t", false, &Vec::new()).0);
    assert!(match_by_char("act", "a?ca?a?t", false, &Vec::new()).0);
    assert!(match_by_char("ct", "a?ca?a?t", false, &Vec::new()).0);
}

#[test]
fn check_optional_type() {
    assert!(match_by_char("act", r"c\d?t", false, &Vec::new()).0);
    assert!(match_by_char("act", r"c\d?t\d?", false, &Vec::new()).0);
}

#[test]
fn check_wildcard() {
    assert!(match_by_char("dog", r"d.g", false, &Vec::new()).0);
    assert!(!match_by_char("2", r".[^abc]", false, &Vec::new()).0);
    assert!(!match_by_char("2", r".[abc]", false, &Vec::new()).0);
    assert!(!match_by_char("2", r".\d", false, &Vec::new()).0);
    assert!(match_by_char("22", r".\d", false, &Vec::new()).0);
    assert!(!match_by_char("dog", r"d.g.", false, &Vec::new()).0);
    assert!(!match_by_char("dog", r".d.g.", false, &Vec::new()).0);
    assert!(!(match_by_char("dog", r".c.g.", false, &Vec::new()).0));
    assert!(!(match_by_char("cog", r"d.g", false, &Vec::new()).0));
}

#[test]
fn check_qty_wildcard() {
    assert!(match_by_char("goøö0Ogol", "g.+gol", false, &Vec::new()).0);
    ////echo -n "gol" | ./your_program.sh -E "g.+gol"
    assert!(!match_by_char("gol", "g.+gol", false, &Vec::new()).0);
}

#[test]
fn failed_before_tester() {
    assert!(!match_by_char("sally has 1 dog", r"\d \w\w\ws", false, &Vec::new()).0);
    assert!(
        match_by_char(
            "I see 1 cat, 2 dogs and 3 cows",
            r"^I see (\d (cat|dog|cow)s?(, | and )?)+$",
            false,
            &Vec::new()
        )
        .0
    );
    ////echo -n "caaats" | ./your_program.sh -E "ca+at"
    assert!(match_by_char("caaats", "ca+at", false, &Vec::new()).0);
    assert!(match_by_char("apple", "[^xyz]", false, &Vec::new()).0);
    assert!(match_by_char("e", "[blueberry]", false, &Vec::new()).0);
    //echo -n "abcd is abcd, not efg" | ./your_program.sh -E "([abcd]+) is \1, not [^xyz]+"
    //echo -n "this starts and ends with this" | ./your_program.sh -E "^(\w+) starts and ends with \1$"
    assert!(
        match_by_char(
            "this starts and ends with this",
            r"^(\w+) starts and ends with \1$",
            false,
            &Vec::new()
        )
        .0
    );
}

#[test]
fn layered_groups() {
    //echo -n "I see 1 cat, 2 dogs and 3 cows" | ./your_program.sh -E "^I see (\d (cat|dog|cow)(, | and )?)+$
    assert!(
        !match_by_char(
            "I see 1 cat, 2 dogs and 3 cows",
            r"^I see (\d (cat|dog|cow)(, | and )?)+$",
            false,
            &Vec::new()
        )
        .0
    )
}

#[test]
fn match_multiple_patterns() {
    assert!(match_by_char("cat", "(cat|dog)", false, &Vec::new()).0);
    assert!(match_by_char("a cat", "a (cat|dog)", false, &Vec::new()).0);
    assert!(match_by_char("a cat is", "a (cat|dog) is", false, &Vec::new()).0);
}

#[test]
fn test_backreference() {
    assert!(match_by_char("cat and cat", r"(cat) and \1", false, &Vec::new()).0);
    assert!(!match_by_char("cat and dog", r"(cat) and \1", false, &Vec::new()).0);
    assert!(!match_by_char("cat and dog", r"(\w+) and \1", false, &Vec::new()).0);
    assert!(match_by_char("cat and cat", r"(\w+) and \1", false, &Vec::new()).0);
    assert!(
        match_by_char(
            "abcd is abcd, not efg",
            r"([abcd]+) is \1, not [^xyz]+",
            false,
            &Vec::new()
        )
        .0
    )
}

#[test]
fn multiple_backref() {
    assert!(
        match_by_char(
            "3 red squares and 3 red circles",
            r"(\d+) (\w+) squares and \1 \2 circles",
            false,
            &Vec::new()
        )
        .0
    )
}

#[test]
fn nested_backref() {
    assert!(
        match_by_char(
            "'cat and cat' is the same as 'cat and cat'",
            r"('(cat) and \2') is the same as \1",
            false,
            &Vec::new()
        )
        .0
    );
    ////echo -n "grep 101 is doing grep 101 times, and again grep 101 times" | ./your_program.sh -E "((\w\w\w\w) (\d\d\d)) is doing \2 \3 times, and again \1 times"
    assert!(
        match_by_char(
            "grep 101 is doing grep 101 times, and again grep 101 times",
            r"((\w\w\w\w) (\d\d\d)) is doing \2 \3 times, and again \1 times",
            false,
            &Vec::new()
        )
        .0
    )
}

#[test]
fn long_backref() {
    ////echo -n "abc-def is abc-def, not efg, abc, or def" | ./your_program.sh -E "(([abc]+)-([def]+)) is \1, not ([^xyz]+), \2, or \3"
    assert!(
        match_by_char(
            "abc-def is abc-def, not efg, abc, or def",
            r"(([abc]+)-([def]+)) is \1, not ([^xyz]+), \2, or \3",
            false,
            &Vec::new()
        )
        .0
    );
    ////echo -n "abc-def is abc-def, not xyz, abc, or def" | ./your_program.sh -E "(([abc]+)-([def]+)) is \1, not ([^xyz]+), \2, or \3"
    assert!(
        !match_by_char(
            "abc-def is abc-def, not xyz, abc, or def",
            r"(([abc]+)-([def]+)) is \1, not ([^xyz]+), \2, or \3",
            false,
            &Vec::new()
        )
        .0
    )
}

#[test]
fn long_backref2() {
    ////echo -n "apple pie is made of apple and pie. love apple pie" | ./your_program.sh -E "^((\w+) (\w+)) is made of \2 and \3. love \1$"
    assert!(
        match_by_char(
            "apple pie is made of apple and pie. love apple pie",
            r"^((\w+) (\w+)) is made of \2 and \3. love \1$",
            false,
            &Vec::new()
        )
        .0
    );
}

#[test]
fn disordered_back_ref() {
    ////echo -n  "cat and fish, cat with fish, cat and fish"| ./your_program.sh -E "((c.t|d.g) and (f..h|b..d)), \2 with \3, \1"
    assert!(
        match_by_char(
            "cat and fish, cat with fish, cat and fish",
            r"((c.t|d.g) and (f..h|b..d)), \2 with \3, \1",
            false,
            &Vec::new()
        )
        .0
    );
}

#[test]
fn tester_failed_2() {
    ////echo -n "apple" | ./your_program.sh -E "[^abc]"
    assert!(match_by_char("apple", r"[^abc]", false, &Vec::new()).0)
}

#[test]
fn test_neg_patt_2() {
    assert!(match_by_char("sabcyeza", r"s[^abc]+eza", false, &Vec::new()).0);
}
