#[cfg(test)]
mod tests;
pub fn match_by_char(input_line: &str, pattern: &str) -> bool {
    let patt_chars: Vec<char> = pattern.chars().collect();
    let input_chars: Vec<char> = input_line.chars().collect();
    let mut patt_index: usize = 0;
    let mut input_index = 0;
    let patt_len = pattern.len();
    let input_len = input_line.len();
    let mut prev_pattern = "";

    if patt_chars[0] == '^' {
        patt_index += 1
    }
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        while patt_index < patt_len && input_index < input_len {
            // eprintln!("start of while input i:{input_index}, pattern i:{patt_index}");
            match patt_chars[patt_index] {
                '[' => {
                    if patt_chars[patt_index + 1..].contains(&']') {
                        let char_group_end = patt_chars[patt_index + 1..]
                            .iter()
                            .position(|c| c == &']')
                            .unwrap();

                        let char_group_length = char_group_end - patt_index;
                        let lett_group =
                            &patt_chars[patt_index + 1..patt_index + char_group_end + 1];
                        prev_pattern = &pattern[patt_index..patt_index + char_group_end + 2];
                        let mut found_pos = 0;
                        eprintln!(
                            "checking char group of length:{char_group_length}, group:{:?}",
                            lett_group
                        );

                        if !(char_group_length > 1 && {
                            if patt_chars[patt_index + 1] != '^' {
                                input_chars[input_index..].iter().enumerate().any(|(i, c)| {
                                    if lett_group.contains(&c) {
                                        found_pos = i;
                                        return true;
                                    } else {
                                        return false;
                                    }
                                })
                            } else {
                                eprintln!("checking negative group");
                                let neg_group = &lett_group[1..];
                                patt_index += 1;
                                input_chars[input_index..].iter().enumerate().any(|(i, c)| {
                                    if !neg_group.contains(&c) {
                                        found_pos = i;
                                        return true;
                                    } else {
                                        return false;
                                    }
                                })
                            }
                        }) {
                            return false;
                        }
                        patt_index += char_group_length + 2;
                        input_index += found_pos + 1;
                    }
                }
                '\\' => {
                    while &pattern[patt_index + 1..patt_index + 2] == r"\" {
                        patt_index += 1;
                    }
                    let char_class = &pattern[patt_index..patt_index + 2];
                    prev_pattern = char_class;
                    eprintln!("checking char class {}", char_class);
                    let curr_remaining = &input_line[input_index..];
                    let mut found_pos = 0;
                    if !(match char_class {
                        r"\d" => curr_remaining.chars().enumerate().any(|(i, e)| {
                            if e.is_digit(10) {
                                found_pos = i;
                                return true;
                            }
                            return false;
                        }),
                        r"\w" => curr_remaining.chars().enumerate().any(|(i, e)| {
                            if e.is_alphanumeric() || e == '_' {
                                found_pos = i;
                                return true;
                            } else {
                                return false;
                            }
                        }),
                        // _ => match_pattern(&curr_remaining, &pattern[patt_index..]),
                        _ => unreachable!(),
                    }) {
                        eprintln!(
                        "returning false in char group, curr patter pos:{patt_index}, input pos:{input_index}"
                    );
                        return false;
                    }
                    patt_index += 2;
                    input_index += found_pos + 1;
                    eprintln!("found a char in group {char_class}, new pos:{input_index}, new patt pos{patt_index}");
                }
                '+' => {
                    let mut similar_remaining_in_pattern = 0;

                    let mut check_index = patt_index + 1;
                    let prev_pattern_len = prev_pattern.len();
                    while check_index < patt_len
                        && &pattern[check_index..check_index + prev_pattern_len] == prev_pattern
                    {
                        eprintln!("checking one");
                        check_index += prev_pattern_len;
                        similar_remaining_in_pattern += 1;
                    }
                    let mut num_repeats = 0;
                    while input_index < input_len
                        && match_by_char(&input_line[input_index..input_index + 1], prev_pattern)
                    {
                        eprintln!("in loop");
                        input_index += 1;
                        num_repeats += 1;
                    }
                    eprintln!("rpts:{num_repeats}, simi:{similar_remaining_in_pattern}, prev_patt_len:{prev_pattern_len}");

                    // if there are more of the same immediately after e.g ca+ats
                    // move pattern pointer forward by at one
                    // move the input index forward by at least 1 * len of prev pattern
                    patt_index += 1;
                    if similar_remaining_in_pattern > 0 {
                        input_index -= std::cmp::max(num_repeats - similar_remaining_in_pattern, 1)
                            * prev_pattern_len;
                        eprintln!("new pattern index,{patt_index}")
                    }
                }
                _ => {
                    prev_pattern = &pattern[patt_index..patt_index + 1];
                    if &pattern[patt_index..patt_index + 1]
                        != &input_line[input_index..input_index + 1]
                    {
                        eprintln!(
                        "returning false in char char mapping, curr patter pos:{patt_index}, input pos:{input_index}"
                    );
                        return false;
                    }

                    patt_index += 1;
                    input_index += 1;
                }
            }
        }
    }

    // if input fully parsed but pattern not exhausted
    if input_index == input_chars.len() {
        eprintln!("final return: input i:{input_index}, patt i:{patt_index}");
        return patt_index >= pattern.len()
            || (patt_index == pattern.len() - 1) && ['$', '?'].contains(&patt_chars[patt_index]);
    };
    true
}
