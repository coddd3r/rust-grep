use std::usize;

use crate::utils::{check_num_similar_pattern, check_optional, get_next_pattern};

#[cfg(test)]
mod tests;
mod utils;

const NULL_RETURN: (bool, Option<usize>, String) = (false, None, String::new());

pub fn match_by_char(
    input_line: &str,
    pattern: &str,
    full_match_optional: bool,
    passed_groups: &Vec<(usize, usize, String)>,
) -> (bool, Option<usize>, String) {
    eprintln!(
        "\n--------------\n----- fn start: MATCHING: input:{:?}, patt:{:?}, full optional?{full_match_optional}",
        input_line, pattern
    );

    let mut patt_capture_groups: Vec<(usize, usize, String)> = Vec::new();
    patt_capture_groups.extend(passed_groups.clone());
    let patt_chars: Vec<char> = pattern.chars().collect();
    //pre-empt optional groups
    if full_match_optional && !patt_chars.contains(&')') && patt_chars.contains(&'|') {
        let mut use_retlen = 0;
        let opt_ret = pattern.split('|').any(|e| {
            let in_res = match_by_char(input_line, e, full_match_optional, &patt_capture_groups);
            if in_res.0 {
                use_retlen = in_res.1.unwrap()
            }
            in_res.0
        });
        return (opt_ret, Some(use_retlen), String::new());
    }

    let input_chars: Vec<char> = input_line.chars().collect();
    let mut patt_index: usize = 0;
    let mut input_index = 0;
    let patt_len = patt_chars.len();
    let input_len = input_chars.len();
    let mut prev_pattern = String::new();
    let mut waiting_groups: Vec<usize> = Vec::new();
    // start, end, string matched

    eprintln!("input length:{input_len}, pattern len:{patt_len}, prev_pattern:{prev_pattern}");
    if pattern.is_empty() {
        panic!("!!!!EMPTY MATCH!!??\n");
        //return (true, Some(0), String::new());
    }

    if input_line.is_empty() {
        panic!("!!!!EMPTY INPUT!!??\n");
        //return NULL_RETURN;
        //return (true, Some(0), String::new());
    }
    let mut starter = false;
    if patt_chars[0] == '^' {
        patt_index += 1;
        starter = true;
    }

    while patt_index < patt_len && input_index < input_len {
        // eprintln!("in loop input:{input_line}, input len:{input_len}, input i:{input_index}");
        // eprintln!("in loop, pattern:{pattern}, patt len:{patt_len}, patt i:{patt_index}");
        //eprintln!("in loop with char{:?}", patt_chars[patt_index]);
        // eprintln!("start of while input i:{input_index}, pattern i:{patt_index}");
        match patt_chars[patt_index] {
            '[' => {
                if patt_chars[patt_index + 1..].contains(&']') {
                    let char_group_end = patt_chars[patt_index + 1..]
                        .iter()
                        .position(|c| c == &']')
                        .unwrap();

                    let lett_group = &patt_chars[patt_index + 1..patt_index + char_group_end + 1];
                    let char_group_length = lett_group.len();
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
                            let neg_group = &lett_group[1..];
                            eprintln!("checking negative group:{:?}", neg_group);
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
                        return NULL_RETURN;
                    }

                    patt_index += char_group_length + 2;
                    if res {
                        input_index += found_pos + 1;
                    }
                    //ignore the '?'
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

                // FIND GROUP, MATCH GROUP
                if char_class.chars().nth(1).unwrap().is_digit(10) {
                    eprintln!(
                        "\n\n\n FOUND back ref:{char_class}, groups:{:?}\n\n",
                        patt_capture_groups
                    );
                    let group_num = char_class[1..].parse::<usize>().unwrap();
                    let actual_group = &patt_capture_groups[group_num - 1].2;
                    let rem_input: String = input_chars[input_index..].iter().collect();
                    let num_group_res = match_by_char(
                        &rem_input,
                        actual_group,
                        full_match_optional,
                        &patt_capture_groups,
                    );

                    if !num_group_res.0 {
                        return num_group_res;
                    }
                    let matched_len = num_group_res.1.unwrap();
                    //patt_capture_groups[group_num - 1].2 =
                    //    input_chars[input_index..matched_len].iter().collect();
                    input_index += matched_len;
                    patt_index += 2;
                    continue;
                }

                prev_pattern = char_class.clone();
                eprintln!("checking char class {}", char_class);
                let curr_remaining = &input_line[input_index..];
                let mut found_pos = 0;
                let is_optional = patt_index + 2 < patt_len && patt_chars[patt_index + 2] == '?';
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
                    eprintln!("NOT FOUND input:{input_line}, pattern:{pattern}");
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
                        eprintln!("returning false in char group, curr patter pos:{patt_index}, input pos:{input_index}");
                        return NULL_RETURN;
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
                eprintln!("\n*************\n***********\nREPEATING PATTERN:{prev_pattern}\n\n\n***********");

                let prev_pattern_len = prev_pattern.len();

                // check how many times the prev matched
                // need to occur in the pattern, after repeat
                let mut num_repeats = 0;
                while input_index < input_len {
                    let use_patt = format!("^{prev_pattern}");
                    let use_input = &input_chars[input_index..].iter().collect::<String>();
                    eprintln!("\n\ncalling repeat,input chars:{:?} input i:{input_index} using input:{:?}, input length{:?}",input_chars, use_input, input_len);
                    let res = match_by_char(&use_input, &use_patt, true, &patt_capture_groups);
                    if !res.0 {
                        break;
                    }
                    eprintln!("in loop res?:{:?}", res);
                    eprintln!("in loop input i?:{:?}", input_index);
                    input_index += res.1.unwrap();
                    num_repeats += 1;
                }

                let similar_remaining_in_pattern = check_num_similar_pattern(
                    patt_index,
                    patt_len,
                    prev_pattern_len,
                    &prev_pattern,
                    &patt_chars,
                    &patt_capture_groups,
                );
                eprintln!("\n\nrpts:{num_repeats}, simi:{similar_remaining_in_pattern}, prev_patt_len:{prev_pattern_len}");

                // if there are more of the same immediately after e.g ca+ats
                // move pattern pointer forward by at one
                // move the input index forward by at least 1 * len of prev pattern
                patt_index += 1;
                eprintln!("original input i before subtr similar:{input_index}");
                if similar_remaining_in_pattern > 0 {
                    if input_index >= input_len {
                        input_index += 1
                    }
                    if num_repeats > similar_remaining_in_pattern {
                        input_index -= std::cmp::max(num_repeats - similar_remaining_in_pattern, 1);
                    } else {
                        input_index -= num_repeats;
                    }
                    eprintln!("ne input:{input_index}");
                }
                eprintln!("AFTER finding all similar: new input i:{input_index}, pattern i:{patt_index}\n");
            }

            '.' => {
                prev_pattern = ".".to_string();
                patt_index += 1;
                input_index += 1;
                continue;
            }

            '(' => {
                // queue the groups waiting to have their capture field populated
                //let mut waiting_group_pos: usize = 0;
                if patt_chars[patt_index + 1..].contains(&')') {
                    patt_capture_groups.push((patt_index, 0, String::new()));
                    let mut num_opening: usize = 1;
                    let mut capture_group_end = 0;
                    let rem_chars = &patt_chars[patt_index + 1..];
                    for (i, e) in rem_chars.iter().enumerate() {
                        if e == &'(' {
                            patt_capture_groups.push((patt_index + 1 + i, 0, String::new()));
                            num_opening += 1;
                        }
                        if e == &')' {
                            eprintln!("num opening:{num_opening}");
                            let num_groups = patt_capture_groups.len();
                            //check for the last position with a zero at .1; means unclosed
                            let zero_pos = patt_capture_groups.iter().rev().position(|e| e.1 == 0);
                            let push_i = num_groups - zero_pos.unwrap() - 1;
                            // have groups wait for a capture by order of the closing brackets
                            waiting_groups.push(push_i);
                            patt_capture_groups[push_i].1 = patt_index + 1 + i;
                            //patt_capture_groups[num_opening - 1].1 = patt_index + 1 + i;
                        }
                        if e == &')' && num_opening == 1 {
                            capture_group_end = i;
                            break;
                        }
                        if e == &')' && num_opening > 1 {
                            num_opening -= 1;
                        }
                    }

                    eprintln!("\nCAPT GROUPS:{:?}\n", patt_capture_groups);
                    eprintln!("waiting positions:{:?}", waiting_groups);

                    let start_capt_input = input_index;
                    let closing_bracket_index = patt_index + capture_group_end + 1;
                    let capt_chars = &patt_chars[patt_index + 1..closing_bracket_index];
                    let capt_group = capt_chars.iter().collect::<String>();

                    if capt_chars[capt_chars.len() - 1] == '+' {
                        let similar_in_patt = check_num_similar_pattern(
                            closing_bracket_index,
                            patt_len,
                            capt_group.len() - 1,
                            &capt_group[..capt_group.len() - 1],
                            &patt_chars,
                            &patt_capture_groups,
                        );
                        eprintln!("\n\n GROUP SIMILAR in patt:{similar_in_patt}");
                    }
                    eprintln!("\n input index:{input_index}, patt_index:{patt_index},capt group:{capt_group}");

                    let group_optional = patt_len > closing_bracket_index
                        && patt_chars[closing_bracket_index] == '?';
                    eprintln!("GROUP optional?{group_optional}");
                    let split_char = {
                        if capt_group.contains('(') {
                            eprintln!("splitting by )");
                            '('
                        } else {
                            '|'
                        }
                    };
                    eprintln!("USING SPLIT CHAR:{split_char}");

                    let mut matched_input_len = 0;
                    if split_char == '(' {
                        eprintln!("IN '(' SPLIT");
                        eprintln!("in layers groups");
                        let split_groups: Vec<_> = capt_group.split(split_char).collect();
                        if !split_groups.iter().all(|e| {
                            eprintln!("\nmatching SPLIT group:{e}");
                            let sub_groups: Vec<_> = e.split_inclusive(')').collect();
                            let res = sub_groups.iter().enumerate().all(|(x, sub_gr)| {
                                eprintln!("\nmatching SUBGROUP:{sub_gr}");
                                let is_closer = sub_gr.chars().last().unwrap() == ')';
                                let next_subgroup = if x + 1 < sub_groups.len() {
                                    sub_groups[x + 1]
                                } else {
                                    ""
                                };
                                let sub_group_optional =
                                    x + 1 < sub_groups.len() && next_subgroup == "?";

                                eprintln!(
                                    "SUB optional?{sub_group_optional}, next grp:{next_subgroup}",
                                );

                                if sub_gr == &"?" || sub_gr.is_empty() {
                                    return true;
                                }

                                let use_patt = {
                                    if is_closer {
                                        let use_patt: Vec<char> =
                                            sub_gr.chars().into_iter().collect();
                                        use_patt[..use_patt.len() - 1].iter().collect()
                                    } else {
                                        format!("{sub_gr}")
                                    }
                                };

                                let sub_group_res = match_by_char(
                                    &input_line[input_index..],
                                    &use_patt,
                                    true,
                                    &patt_capture_groups,
                                );

                                if !sub_group_res.0 {
                                    eprintln!("\nsub group:{sub_gr} NOT found\n");
                                } else {
                                    eprintln!(
                                        "\nsub group:{sub_gr} FOUND, res:{:?}\n",
                                        sub_group_res
                                    );
                                    eprintln!("all groups:{:?}", patt_capture_groups);
                                    if is_closer {
                                        let use_pos = waiting_groups.remove(0);
                                        eprintln!(
                                            "for patter:{pattern} waiting group pos:{use_pos}"
                                        );
                                        patt_capture_groups[use_pos].2 = sub_group_res.2;
                                    }
                                }

                                if sub_group_optional && !sub_group_res.0 {
                                    eprintln!("\nsubgroup NOT found but OPTIONAL");
                                    return true;
                                }

                                if sub_group_res.0 {
                                    eprintln!(
                                        "moving input forward by {}",
                                        sub_group_res.1.unwrap()
                                    );
                                    input_index += sub_group_res.1.unwrap();
                                    matched_input_len = sub_group_res.1.unwrap();
                                }

                                eprintln!("\n\nSUB GROUP:{e}, returning:{}\n\n", sub_group_res.0);
                                sub_group_res.0
                            });
                            eprintln!("\n\nSPLIT GROUP:{e}, returning:{res}\n\n");
                            res
                        }) {
                            eprintln!("\n\n'(' SPLIT groups matching false for input:{input_line}, patt:{pattern}\n\n");
                            return NULL_RETURN;
                        }
                    } else if patt_chars[patt_index] == '('
                        && !patt_chars[patt_index + 1..].contains(&')')
                    {
                        //TODO: handle straggler (
                        patt_index += 1;
                        continue;
                    } else {
                        eprintln!("\n\nIN SECOND SPLIT\n");
                        let mut split_groups = capt_group.split(split_char);

                        if !split_groups.any(|e| {
                            eprintln!("matching GROUP:{e}");
                            let res = match_by_char(
                                &input_line[input_index..],
                                e,
                                true,
                                &patt_capture_groups,
                            );
                            if res.0 {
                                matched_input_len = res.1.unwrap();
                            }
                            eprintln!("\n\nGROUP RESULT:{:?}", res);
                            res.0
                        }) {
                            eprintln!("\nSECOND SPLIT groups matching false for input:{input_line}, patt:{pattern}\n");
                            return NULL_RETURN;
                        }
                        eprintln!("group return matched len:{matched_input_len}");

                        input_index += matched_input_len;
                        eprintln!(
                            "AFTER MULTIPLE input index:{input_index}, patt_index:{patt_index}"
                        );
                    }
                    patt_index += capt_group.len() + 2;
                    let captured_input: String =
                        input_chars[start_capt_input..input_index].iter().collect();
                    eprintln!("\n\n\nCAPTURED:{captured_input} for pattern:{capt_group}");

                    let use_pos = waiting_groups.remove(0);
                    eprintln!("for patter:{pattern} waiting group pos:{use_pos}");
                    patt_capture_groups[use_pos].2 = captured_input;
                    eprintln!("\n\nALL CAPT GROUPS:{:?}", patt_capture_groups);
                    //patt_capture_groups[0].2 = captured_input;
                    eprintln!("input:{input_line}, input i:{input_index}, pattern:{pattern}, patt i:{patt_index}");
                    prev_pattern = capt_group;
                }
            }

            _ => {
                if ['?'].contains(&patt_chars[patt_index]) {
                    patt_index += 1;
                    continue;
                }
                if patt_chars[patt_index] == '$' && patt_index != patt_len - 1 {
                    eprintln!("FALSE END");
                    return NULL_RETURN;
                }
                prev_pattern = patt_chars[patt_index..patt_index + 1]
                    .into_iter()
                    .collect::<String>();
                let is_optional = patt_index + 1 < patt_len && patt_chars[patt_index + 1] == '?';
                let res = patt_chars[patt_index] == input_chars[input_index];

                //if char is not found but the next part of the pattern is optional
                if !res && !is_optional {
                    eprintln!("NOT FOUND input:{input_line}, pattern:{pattern}");
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
                        "returning false in char comp input:{} patt:{} mapping, curr patter pos:{patt_index}, input pos:{input_index}",
                                input_chars[input_index], patt_chars[patt_index]
                    );
                        return NULL_RETURN;
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

    if starter && input_chars.len() > 1 {
        let next_patt = get_next_pattern(pattern);
        //let next_patt = get_next_pattern(&patt_chars[next_patt.len()..].iter().collect::<String>());
        eprintln!("\n\n\nCHECKING STARTER: next:{next_patt}");

        if !(match_by_char(
            &input_chars[0..1].iter().collect::<String>(),
            &next_patt,
            full_match_optional,
            &patt_capture_groups,
        )
        .0)
        {
            eprintln!("STARTERT false");
            return NULL_RETURN;
        }
    }

    eprintln!("AFTER WHILE input i:{input_index}");
    let matched_input: String = input_chars[..input_index].iter().collect();
    // if matching  partial return the length matched
    if full_match_optional && patt_index == pattern.len() {
        let ret_len = input_index;
        eprintln!(
            "BEFORE RETURN TRUE full optional:{full_match_optional}; input:{input_line},pattern{:?} returning an input length of:{}",
            pattern, ret_len
        );
        let ret = (true, Some(ret_len), matched_input);
        eprintln!("returning:{:?}", ret);
        return ret;
    }

    if patt_index == patt_len {
        eprintln!("returning TRUE patt fully parsed");
        return (true, Some(input_index), matched_input);
    }
    // if input fully parsed but pattern not exhausted
    if input_index == input_chars.len() {
        //optional full parse
        //OR pattern fully parsed
        //OR pattern is optional/end marker
        //OR the remaining pattern is optional

        eprintln!("INPUT FULLY PARSED full optional:{full_match_optional}");
        if full_match_optional {
            return (true, Some(input_len), matched_input);
        }
        let res = patt_index >= patt_len
            || ((patt_index == patt_len - 1) && ['$', '?', '+'].contains(&patt_chars[patt_index]))
            || patt_len - patt_index > 1 && {
                let remaining_patt = patt_chars[patt_index..].into_iter().collect::<String>();
                check_optional(&remaining_patt)
            };
        eprintln!("final return: input:{input_line}, i:{input_index}, input len:{input_len}\n  patt len:{patt_len}, pattern:{pattern}, patt i:{patt_index},\nres:{res}");
        return (res, Some(input_len), matched_input);
    };
    eprintln!("\n\n:( COP OUT TRUE");
    eprintln!(
        "input:{input_line}, pattern:{pattern}, input pos:{input_index}, patt_pos:{patt_index}"
    );
    eprintln!("input len:{input_len}, pattern len:{patt_len}, ");
    return NULL_RETURN;
}
