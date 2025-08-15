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
    // pass capture groups for each recursion
    patt_capture_groups.extend(passed_groups.clone());
    let patt_chars: Vec<char> = pattern.chars().collect();

    //pre-empt recurring optional groups without brackets around them
    if !patt_chars.contains(&')') && patt_chars.contains(&'|') {
        let mut use_retlen = 0;
        let mut use_capt = String::new();
        let opt_ret = pattern.split('|').any(|e| {
            let in_res = match_by_char(input_line, e, full_match_optional, &patt_capture_groups);
            if in_res.0 {
                use_retlen = in_res.1.unwrap();
                use_capt = in_res.2;
            }
            in_res.0
        });
        return (opt_ret, Some(use_retlen), use_capt);
    }

    let input_chars: Vec<char> = input_line.chars().collect();
    let mut patt_index: usize = 0;
    let mut input_index = 0;
    let patt_len = patt_chars.len();
    let input_len = input_chars.len();
    let mut prev_pattern = String::new();
    //queue of capture groups waiting to be assigned
    //a string matched from input
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
        eprintln!(
            "in loop input:{input_line}, input len:{input_len}, input i:{input_index}, char:{:?}",
            patt_chars[patt_index]
        );
        eprintln!(
            "in loop, pattern:{pattern}, patt len:{patt_len}, patt i:{patt_index}, char:{:?}",
            patt_chars[patt_index]
        );

        match patt_chars[patt_index] {
            '[' => {
                if patt_chars[patt_index + 1..].contains(&']') {
                    let char_group_end = patt_chars[patt_index + 1..]
                        .iter()
                        .position(|c| c == &']')
                        .unwrap();

                    let lett_group = &patt_chars[patt_index + 1..patt_index + char_group_end + 1];
                    let char_group_length = lett_group.len();
                    //make sure prev pattern includes the brackets []
                    prev_pattern = patt_chars[patt_index..patt_index + char_group_end + 2]
                        .into_iter()
                        .collect::<String>();

                    let mut found_pos = 0;
                    //eprintln!(
                    //    "checking char group of length:{char_group_length}, group:{:?}",
                    //    lett_group
                    //);

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
                                    eprintln!("is true");
                                    return true;
                                } else {
                                    eprintln!("false");
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

                    eprintln!("opt:{is_optional}, res:{res}, patt index:{patt_index}, input index:{input_index}");
                }
            }

            '\\' => {
                while patt_chars[patt_index + 1] == '\\' {
                    patt_index += 1;
                }
                let char_class = patt_chars[patt_index..patt_index + 2]
                    .into_iter()
                    .collect::<String>();

                // FIND char class
                if char_class.chars().nth(1).unwrap().is_digit(10) {
                    eprintln!(
                        "\n\n FOUND back ref:{char_class}, groups:{:?}\n",
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
                            "Patt to check:{patt_to_check});
                eprintln!( checking next optional?{next_optional}"
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

                //full number of times the prev matched can occur
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

                // check how many matches similar to the prev matched
                // need to occur in the pattern, after repeat
                let similar_remaining_in_pattern = check_num_similar_pattern(
                    patt_index,
                    &prev_pattern,
                    &patt_chars,
                    &patt_capture_groups,
                );
                eprintln!("\n\nrpts:{num_repeats}, simi:{similar_remaining_in_pattern},prev_patt:{prev_pattern}");

                // if there are more of the same immediately after e.g "ca+ats" for "caaaats"
                // move pattern pointer forward by one
                // move the input index forward by at least 1
                // or as many as possible while still satisfying the rest of the pattern
                patt_index += 1;
                if similar_remaining_in_pattern > num_repeats {
                    eprintln!("REMAINING pattern has too many subpatterns");
                    return NULL_RETURN;
                }

                if similar_remaining_in_pattern > 0 {
                    input_index -= similar_remaining_in_pattern;
                    eprintln!("new input index:{input_index}");
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
                let initial_input_i = input_index;
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
                        }

                        if e == &')' && num_opening == 1 {
                            capture_group_end = i;
                            break;
                        }

                        if e == &')' && num_opening > 1 {
                            num_opening -= 1;
                        }
                    }

                    eprintln!("\nCAPT GROUPS:{:?}", patt_capture_groups);
                    eprintln!("waiting positions:{:?}", waiting_groups);

                    let start_capt_input = input_index;
                    let closing_bracket_index = patt_index + capture_group_end + 1;
                    let capt_chars = &patt_chars[patt_index + 1..closing_bracket_index];
                    let capt_group = capt_chars.iter().collect::<String>();

                    //if group repeated, check after this repeated group
                    //how many of the remaning subpattern types would match the same
                    //as this repeated group
                    let mut similar_in_patt = 0;
                    if capt_chars[capt_chars.len() - 1] == '+' {
                        similar_in_patt = check_num_similar_pattern(
                            closing_bracket_index,
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
                            '('
                        } else {
                            '|'
                        }
                    };
                    eprintln!("USING SPLIT CHAR:{split_char}");

                    let mut matched_input_len = 0;
                    if split_char == '(' {
                        eprintln!("IN '(' SPLIT");
                        eprintln!("in layered groups");
                        let split_groups: Vec<_> = capt_group.split(split_char).collect();
                        if !split_groups.iter().all(|e| {
                            eprintln!("\nmatching SPLIT group:{e}");

                            //split by closing brackets to get all subgroups
                            let sub_groups: Vec<_> = e.split_inclusive(')').collect();
                            let res = sub_groups.iter().enumerate().all(|(x, sub_gr)| {
                                eprintln!("\nmatching SUBGROUP:{sub_gr}");

                                //if subgroup is catured
                                //split inclusive will leave its closing bracket in place
                                let is_closer = sub_gr.chars().last().unwrap() == ')';
                                let next_subgroup = if x + 1 < sub_groups.len() {
                                    sub_groups[x + 1]
                                } else {
                                    ""
                                };
                                let sub_group_optional =
                                    x + 1 < sub_groups.len() && next_subgroup == "?";

                                eprintln!("SUB optional?{sub_group_optional}");
                                eprintln!("next grp:{next_subgroup}");

                                //in case of an optional marker or if the split gives an empty str
                                if sub_gr == &"?" || sub_gr.is_empty() {
                                    return true;
                                }

                                let use_patt = {
                                    //in captured subgroup, remove the last backet before recurring
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

                                    //if captured, use its returned matched input
                                    //in its corresponding group
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
                            if !group_optional {
                                return NULL_RETURN;
                            }
                        }
                    }
                    //if no subgroups split by '|'
                    else {
                        eprintln!("\n\nIN SECOND SPLIT pattern:{pattern} patt i:{patt_index}\n input:{input_line}, input i:{input_index}\n");
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
                        eprintln!("AFTER '|' split input i:{input_index},patt_i:{patt_index}");
                    }

                    //move past capt group
                    patt_index += capt_group.len() + 2;

                    eprintln!("removing similar in capt groups:{similar_in_patt} from input index:{input_index}");
                    // remove matches that could be matched later in the pattern
                    // in case patter was repeated, and there are similar matched right after
                    // move back n times to make sure the rest of the pattern can be matched
                    input_index -= similar_in_patt;

                    //SPECIAL CASE for '[]' groups
                    //make sure the range left with after subtracting the similar
                    //actually matches the pattern e.g "[^xyz]."  and "xyza"
                    //will match up to a for the group but will be moved back by one
                    //to accommodate the '.', the result should be false
                    let supposed_matched: String =
                        input_chars[initial_input_i..input_index].iter().collect();
                    if !match_by_char(&supposed_matched, &capt_group, false, &patt_capture_groups).0
                    {
                        eprintln!("\n\n\nFAILING supposed match");
                        eprintln!("supposed:{supposed_matched}, capt:{capt_group}");
                        return NULL_RETURN;
                    }

                    let captured_input: String =
                        input_chars[start_capt_input..input_index].iter().collect();
                    eprintln!("\n\n\nCAPTURED:{captured_input} for pattern:{capt_group}");

                    let use_pos = waiting_groups.remove(0);
                    eprintln!("for patter:{pattern} waiting group pos:{use_pos}");
                    patt_capture_groups[use_pos].2 = captured_input;
                    eprintln!("\n\nALL CAPT GROUPS:{:?}", patt_capture_groups);
                    eprintln!("input:{input_line}, input i:{input_index}\n, pattern:{pattern}, patt i:{patt_index}");
                    prev_pattern = capt_group;
                } else if patt_chars[patt_index] == '('
                    && !patt_chars[patt_index + 1..].contains(&')')
                {
                    panic!();
                    //TODO: handle straggler (
                }
            }

            '$' => {
                eprintln!("found end, input i:{input_index}, input len:{input_len}");
                if input_index != input_len - 1 {
                    eprintln!("FALSE END");
                    return NULL_RETURN;
                }
                eprintln!("RETURNING AT END");
                return (true, Some(input_len), input_line.to_string());
            }

            _ => {
                if ['?'].contains(&patt_chars[patt_index]) {
                    patt_index += 1;
                    continue;
                }

                //false end
                if patt_chars[patt_index] == '$' && patt_index != patt_len - 1 {
                    eprintln!("FALSE END");
                    return NULL_RETURN;
                }

                prev_pattern = patt_chars[patt_index..patt_index + 1]
                    .into_iter()
                    .collect::<String>();

                let is_optional = patt_index + 1 < patt_len && patt_chars[patt_index + 1] == '?';
                //direct char to char match
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
                        eprintln!("Patt to check:{remaining_patt}");
                        eprintln!("checking next optional?{next_optional}");
                    }

                    if next_optional
                        || (patt_index + 1 < patt_len
                            && ['?', '+', '$'].contains(&patt_chars[patt_index + 1]))
                    {
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
                //skip '?'
                if is_optional {
                    patt_index += 1;
                }

                patt_index += 1;
            }
        }
    }

    //if '^' at the start of pattern, make sure the pattern directly at start
    //matches the first pattern
    if starter && input_chars.len() > 1 {
        let next_patt = get_next_pattern(pattern);
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

    if patt_index == patt_len && patt_chars.last().unwrap() != &'$' {
        eprintln!("returning TRUE patt fully parsed input i:{input_index}, input len:{input_len}");
        return (true, Some(input_index), matched_input);
    }

    // if input fully parsed but pattern not exhausted
    if input_index == input_chars.len() {
        //optional full parse
        //OR pattern fully parsed
        //OR pattern is optional/end marker
        //OR the remaining pattern is optional
        //or remainder is '?' or '*' followed by '$'

        eprintln!("INPUT FULLY PARSED full optional:{full_match_optional}");
        if full_match_optional {
            return (true, Some(input_len), matched_input);
        }

        let res = patt_index >= patt_len
            || ((patt_index == patt_len - 1)
                && ['$', '?', '+', '*'].contains(&patt_chars[patt_index]))
            || patt_len - patt_index > 1 && {
                let remaining_chars = &patt_chars[patt_index..];
                let remaining_patt = remaining_chars.into_iter().collect::<String>();
                remaining_chars.len() == 2
                    && ['?', '*'].contains(&remaining_chars[0])
                    && ['$'].contains(&remaining_chars[1])
                    || check_optional(&remaining_patt)
            };
        eprintln!("final return: input:{input_line}, i:{input_index}, input len:{input_len}\n  patt len:{patt_len}, pattern:{:#?}, patt i:{patt_index},\nres:{res}", pattern);
        return (res, Some(input_len), matched_input);
    };

    eprintln!("\n\n:( RETURNING FALSE");
    eprintln!("input:{input_line}, pattern:{pattern},input i:{input_index},patt_i:{patt_index}");
    eprintln!("input len:{input_len}, pattern len:{patt_len}, ");
    return NULL_RETURN;
}
