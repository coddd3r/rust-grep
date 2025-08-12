#[cfg(test)]
mod tests;
pub fn match_by_char(input_line: &str, pattern: &str) -> bool {
    let patt_chars: Vec<char> = pattern.chars().collect();
    let input_chars: Vec<char> = input_line.chars().collect();
    let mut patt_index: usize = 0;
    let mut input_index = 0;
    let patt_len = pattern.len();
    let input_len = input_line.len();
    let mut prev_pattern = String::new();

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
                        prev_pattern = patt_chars[patt_index..patt_index + char_group_end + 2]
                            .into_iter()
                            .collect::<String>();
                        let mut found_pos = 0;
                        eprintln!(
                            "checking char group of length:{char_group_length}, group:{:?}",
                            lett_group
                        );

                        let optional_position = patt_index + char_group_end + 2;
                        let is_optional =
                            optional_position < patt_len && patt_chars[optional_position] == '?';

                        let res = char_group_length > 1 && {
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
                        };

                        if !is_optional && !res {
                            return false;
                        }

                        patt_index += char_group_length + 2;
                        if res {
                            input_index += found_pos + 1;
                        }
                        if is_optional {
                            patt_index += 1;
                        }

                        eprintln!(
                            "opt:{is_optional}, found res:{res}, patt index:{patt_index}, input index:{input_index}"
                        );
                    }
                }
                '\\' => {
                    while patt_chars[patt_index + 1] == '\\' {
                        patt_index += 1;
                    }
                    let char_class = patt_chars[patt_index..patt_index + 2]
                        .into_iter()
                        .collect::<String>();
                    prev_pattern = char_class.clone();
                    eprintln!("checking char class {}", char_class);
                    let curr_remaining = &input_line[input_index..];
                    let mut found_pos = 0;
                    let is_optional =
                        patt_index + 2 < patt_len && patt_chars[patt_index + 2] == '?';
                    let res = match char_class.as_str() {
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
                        _ => unreachable!(),
                    };
                    if !res && !is_optional {
                        let mut next_optional = false;
                        if patt_len - patt_index > 2 {
                            let patt_to_check =
                                &patt_chars[patt_index + 1..].into_iter().collect::<String>();
                            next_optional = check_optional(patt_to_check);
                            eprintln!(
                            "Patt to check: {patt_to_check}, checking next with next optional?{}",
                            next_optional
                        );
                        }
                        if next_optional {
                            input_index += 1;
                            continue;
                        } else {
                            eprintln!(
                        "returning false in char group, curr patter pos:{patt_index}, input pos:{input_index}"
                    );
                            return false;
                        }
                    }
                    patt_index += 2;
                    if res {
                        input_index += found_pos + 1;
                    }
                    if is_optional {
                        patt_index += 1;
                    }
                    eprintln!("found a char in group {char_class}, new pos:{input_index}, new patt pos{patt_index}");
                }
                '+' => {
                    let mut similar_remaining_in_pattern = 0;

                    let mut check_index = patt_index + 1;
                    let prev_pattern_len = prev_pattern.len();
                    while check_index < patt_len
                        && patt_chars[check_index..check_index + prev_pattern_len]
                            .into_iter()
                            .collect::<String>()
                            == prev_pattern
                    {
                        eprintln!("checking one");
                        check_index += prev_pattern_len;
                        similar_remaining_in_pattern += 1;
                    }
                    let mut num_repeats = 0;
                    while input_index < input_len
                        && match_by_char(&input_line[input_index..input_index + 1], &prev_pattern)
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

                '.' => {
                    let mut remaining_patt =
                        patt_chars[patt_index + 1..].into_iter().collect::<String>();

                    let mut next_patt = get_next_pattern(&remaining_patt);
                    eprintln!("next patt:{}", next_patt);
                    let mut one_or_more = false;
                    //let mut any_or_more = false;
                    if ["+", "*"].contains(&next_patt) {
                        one_or_more = next_patt == "+";
                        //any_or_more = next_patt == "*";
                        patt_index += 1;
                        remaining_patt =
                            patt_chars[patt_index + 1..].into_iter().collect::<String>();
                        next_patt = get_next_pattern(&remaining_patt);
                    }
                    let mut num_wild_matches = 0;
                    while input_index < input_len
                        && !match_by_char(
                            &input_chars[input_index..input_index + 1]
                                .into_iter()
                                .collect::<String>(),
                            next_patt,
                        )
                    {
                        input_index += next_patt.len();
                        num_wild_matches += 1;
                    }

                    if num_wild_matches == 0 && one_or_more {
                        return false;
                    }
                    patt_index += 1;
                    //return false;
                    continue;
                }
                _ => {
                    prev_pattern = patt_chars[patt_index..patt_index + 1]
                        .into_iter()
                        .collect::<String>();
                    let is_optional =
                        patt_index + 1 < patt_len && patt_chars[patt_index + 1] == '?';
                    let res = patt_chars[patt_index] == input_chars[input_index];
                    eprintln!("optional char?{is_optional}");

                    //if char is not found but the next part of the pattern is optional
                    if !res && !is_optional {
                        let mut next_optional = false;
                        //if the next part of the pattern is optional
                        if patt_len - patt_index > 2 {
                            let remaining_patt =
                                patt_chars[patt_index + 1..].into_iter().collect::<String>();
                            next_optional = check_optional(&remaining_patt);
                            eprintln!(
                            "Patt to check: {remaining_patt}, checking next with next optional?{}",
                            next_optional
                        );
                        }
                        if next_optional {
                            input_index += 1;
                            continue;
                        } else {
                            eprintln!(
                        "returning false in char char mapping, curr patter pos:{patt_index}, input pos:{input_index}"
                    );
                            return false;
                        }
                    }
                    if res {
                        input_index += 1;
                    }
                    if is_optional {
                        patt_index += 1;
                    }

                    patt_index += 1;
                }
            }
        }
    }

    // if input fully parsed but pattern not exhausted
    if input_index == input_chars.len() {
        eprintln!("final return: input i:{input_index}, patt i:{patt_index}");
        //pattern fully parsed
        //OR pattern is optional/end marker
        //OR the remaining pattern is optional

        return patt_index >= pattern.len()
            || ((patt_index == pattern.len() - 1)
                && ['$', '?', '.'].contains(&patt_chars[patt_index]))
            || patt_len - patt_index > 1 && {
                let remaining_patt = patt_chars[patt_index..].into_iter().collect::<String>();
                check_optional(&remaining_patt)
            };
    };
    true
}

fn check_optional(pattern: &str) -> bool {
    let mut patt_index: usize = 0;
    let patt_len = pattern.len();
    let patt_chars: Vec<char> = pattern.chars().collect();

    match patt_chars[patt_index] {
        '[' => {
            if patt_chars[patt_index + 1..].contains(&']') {
                let char_group_end = patt_chars[patt_index + 1..]
                    .iter()
                    .position(|c| c == &']')
                    .unwrap();
                let optional_position = patt_index + char_group_end + 2;
                optional_position < patt_len && patt_chars[optional_position] == '?'
            } else {
                false
            }
        }
        '\\' => {
            while &patt_chars[patt_index + 1] == &'\\' {
                patt_index += 1;
            }
            let char_class = &patt_chars[patt_index..patt_index + 2]
                .into_iter()
                .collect::<String>();
            eprintln!("checking char class {}", char_class);
            patt_index + 2 < patt_len && patt_chars[patt_index + 2] == '?'
        }
        _ => {
            let opt_position = patt_chars[patt_index + 1];
            eprintln!("opt position:{opt_position}");
            patt_index + 1 < patt_len && opt_position == '?'
        }
    }
}

fn get_next_pattern(pattern: &str) -> &str {
    let mut patt_index: usize = 0;
    let patt_len = pattern.len();
    let patt_chars: Vec<char> = pattern.chars().collect();

    match patt_chars[patt_index] {
        '[' => {
            let char_group_end = patt_chars[patt_index + 1..]
                .iter()
                .position(|c| c == &']')
                .unwrap();
            &pattern[patt_index..patt_index + char_group_end + 2]
        }
        '\\' => {
            while (patt_index + 1) < patt_len && &pattern[patt_index + 1..patt_index + 2] == r"\" {
                patt_index += 1;
            }
            &pattern[patt_index..patt_index + 2]
        }
        _ => &pattern[patt_index..patt_index + 1],
    }
}
