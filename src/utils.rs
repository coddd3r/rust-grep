use crate::match_by_char;

pub fn check_optional(pattern: &str) -> bool {
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

        '(' => {
            if patt_chars[patt_index + 1..].contains(&')') {
                let mut num_opening: usize = 1;
                let mut capture_group_end = 0;
                let rem_chars = &patt_chars[patt_index + 1..];
                for (i, e) in rem_chars.iter().enumerate() {
                    if e == &'(' {
                        num_opening += 1;
                    }

                    if e == &')' && num_opening == 1 {
                        capture_group_end = i;
                        break;
                    }

                    if e == &')' && num_opening > 1 {
                        num_opening -= 1;
                    }
                }
                let closing_bracket_index = patt_index + capture_group_end + 1;
                patt_len > closing_bracket_index && patt_chars[closing_bracket_index] == '?'
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
            if ['?'].contains(&patt_chars[patt_index]) {
                patt_index += 1;
            }
            let opt_position = patt_chars[patt_index + 1];
            eprintln!("opt position:{opt_position}");
            patt_index + 1 < patt_len && opt_position == '?'
        }
    }
}

pub fn get_next_pattern(pattern: &str) -> String {
    let mut patt_index: usize = 0;
    let patt_len = pattern.len();
    let patt_chars: Vec<char> = pattern.chars().collect();

    while patt_index < patt_len {
        match patt_chars[patt_index] {
            '[' => {
                let char_group_end = patt_chars[patt_index + 1..]
                    .iter()
                    .position(|c| c == &']')
                    .unwrap();
                return pattern[patt_index..patt_index + char_group_end + 2].to_string();
            }

            '\\' => {
                while (patt_index + 1) < patt_len
                    && &pattern[patt_index + 1..patt_index + 2] == r"\"
                {
                    patt_index += 1;
                }
                return pattern[patt_index..patt_index + 2].to_string();
            }

            '(' => {
                if patt_chars[patt_index + 1..].contains(&')') {
                    let mut num_opening: usize = 1;
                    let rem_chars = &patt_chars[patt_index + 1..];
                    let mut first_start = patt_index;
                    let mut first_end = 0;
                    eprintln!("before group loop, patt i:{patt_index}");
                    //next pattern will be the first closed group
                    for (i, e) in rem_chars.iter().enumerate() {
                        if e == &'(' {
                            num_opening += 1;
                            eprintln!("setting first start to:{i}");
                            first_start = patt_index + 1 + i;
                        }
                        if e == &')' {
                            eprintln!("num opening:{num_opening}");
                            first_end = patt_index + 1 + i;
                            break;
                        }
                    }

                    let opening_bracket = first_start;
                    let closing_bracket_index = first_end;
                    eprintln!("findin next patt, is group,  opening:{opening_bracket}, closing:{closing_bracket_index}");
                    let capt_chars = &patt_chars[opening_bracket + 1..closing_bracket_index];
                    let capt_group = capt_chars.iter().collect::<String>();
                    eprintln!("from patt:{pattern} returning next group:{capt_group}");
                    return capt_group;
                }
            }

            '^' | '+' | ']' | ')' => patt_index += 1,

            _ => return pattern[patt_index..patt_index + 1].to_string(),
        }
    }
    return String::new();
}

pub fn check_num_similar_pattern(
    patt_index: usize,
    prev_pattern: &str,
    patt_chars: &Vec<char>,
    patt_capture_groups: &Vec<(usize, usize, String)>,
) -> usize {
    let mut check_index = patt_index + 1;
    let mut similar_remaining_in_pattern = 0;
    eprintln!(
        "\n________\nCHECKING SIMILAR PATTERN: patt i:{check_index}, patt_chars:{:?}",
        patt_chars
    );
    let patt_len = patt_chars.len();
    eprintln!("before while, check i:{check_index}, patt len:{patt_len}");
    while check_index < patt_len {
        let next_pattern = get_next_pattern(&patt_chars[check_index..].iter().collect::<String>());
        let prev_pattern_len = prev_pattern.len();
        eprintln!("\nchecking repeat with next pattern:{next_pattern}");

        // handle checking repeats in capture groups
        if next_pattern.chars().nth(0).unwrap() == '\\' {
            eprintln!("\n\n\n\n\n\n****SIMILAR CAPTURE: with capt:{next_pattern}");
            let capt_gr_num = next_pattern.chars().nth(1).unwrap().to_digit(10).unwrap();
            let captured = &patt_capture_groups[capt_gr_num as usize - 1].2;
            similar_remaining_in_pattern += check_num_similar_pattern(
                0,
                prev_pattern,
                &captured.chars().collect(),
                patt_capture_groups,
            )
        }

        //if there is an exact similar to the prev matched pattern
        let opt_index = check_index + next_pattern.len();
        let opt_pattern = opt_index < patt_chars.len()
            && get_next_pattern(&patt_chars[opt_index..].iter().collect::<String>()) == "?";

        // ignore optional patterns
        if opt_pattern {
            check_index += prev_pattern_len;
            continue;
        }

        if next_pattern == *prev_pattern {
            eprintln!("checking full patt");
            check_index += prev_pattern_len;
            similar_remaining_in_pattern += 1;
            continue;
        }

        // if the next char in the pattern matches the pattern also e.g "\d" and '2'
        if match_by_char(&next_pattern, &prev_pattern, false, patt_capture_groups).0 {
            eprintln!("checking one patt char");
            check_index += next_pattern.len();
            similar_remaining_in_pattern += 1;
            continue;
        }
        break;
    }
    eprintln!("RETURNING SIMILAR :{similar_remaining_in_pattern}");
    similar_remaining_in_pattern
}
