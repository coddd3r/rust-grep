#[cfg(test)]
mod tests;
pub fn match_by_char(
    input_line: &str,
    pattern: &str,
    full_match_optional: bool,
) -> (bool, Option<usize>) {
    eprintln!("\n--------------\n----- fn start: MATCHING: input:{input_line}, patt:{:?}", pattern);
    let patt_chars: Vec<char> = pattern.chars().collect();
    let input_chars: Vec<char> = input_line.chars().collect();
    let mut patt_index: usize = 0;
    let mut input_index = 0;
    let patt_len = pattern.len();
    let input_len = input_line.len();
    let mut prev_pattern = String::new();

    eprintln!("input length:{input_len}, pattern len:{patt_len}, prev_pattern:{prev_pattern}");
    if pattern.is_empty() {
        eprintln!("\n\n!!!!EMPTY MATCH!!??\n");
        return (true, Some(0));
    }

    if patt_chars[0] == '^' {
        patt_index += 1
    }
    if pattern.chars().count() == 1 {
        return (input_line.contains(pattern), Some(1));
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
                            return (false, None);
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
                            return (false, None);
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
                    let mut similar_remaining_in_pattern = 0;

                    let mut check_index = patt_index + 1;
                    let prev_pattern_len = prev_pattern.len();
                    while check_index+prev_pattern_len < patt_len
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
                    {
                        let res =   match_by_char(
                            &input_line[input_index..],
                            &prev_pattern,
                            true
                        );
                        if !res.0 {break;}
                        eprintln!("in loop res?:{}", res.0);
                        input_index += res.1.unwrap();
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
                            false,
                        )
                        .0
                    {
                        input_index += next_patt.len();
                        num_wild_matches += 1;
                    }

                    if num_wild_matches == 0 && one_or_more {
                        return (false, None);
                    }
                    patt_index += 1;
                    //return false;
                    continue;
                }
                '(' => {
                    if patt_chars[patt_index + 1..].contains(&')') {
                        let mut num_opening: usize = 1;
                        let mut capture_group_end = 0;
                        let rem_chars = &patt_chars[patt_index + 1..];
                        for (i, e) in rem_chars.iter().enumerate() {
                            if e == &'(' {
                                num_opening += 1
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
                        let capt_group = patt_chars[patt_index + 1..closing_bracket_index]
                            .into_iter()
                            .collect::<String>();
                        eprintln!("\n input index:{input_index}, patt_index:{patt_index},capt group:{capt_group}");

                        let group_optional = patt_len > closing_bracket_index
                            && patt_chars[closing_bracket_index] == '?';
                        eprintln!("\nGROUP optional?{group_optional}");
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
                            eprintln!("\n\nIN '(' SPLIT\n");
                            eprintln!("in layers groups");
                            let split_groups: Vec<_> = capt_group.split(split_char).collect();
                            if !split_groups.iter().all(| e| {
                                eprintln!("\nmatching group:{e}");
                                let sub_groups: Vec<_> = e.split(')').collect();
                                let res = sub_groups.iter().enumerate().all(|(x,sub_gr)| {
                                    eprintln!("\nmatching SUBGROUP:{sub_gr}");
                                    let next_subgroup = if  x + 1 < sub_groups.len() {sub_groups[x + 1]} else {""};
                                    let sub_group_optional =
                                        x + 1 < sub_groups.len() && next_subgroup == "?";

                                    eprintln!("SUB optional?{sub_group_optional}, next grp:{next_subgroup}",);

                                    if sub_gr == &"?" || sub_gr.is_empty() {
                                        return true;
                                    }
                                    let use_patt = format!("({sub_gr})");
                                    let sub_group_res =
                                        match_by_char(&input_line[input_index..], &use_patt, true);

                                    if !sub_group_res.0 {
                                        eprintln!("\n\n\nsub group:{sub_gr} NOT found\n\n");
                                    } else{
                                        
                                        eprintln!("\n\n\nsub group:{sub_gr} FOUND\n\n");
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
                                return (false, None);
                            }
                        } else {
                            eprintln!("\n\n\nIN SECOND SPLIT\n\n");
                            let mut split_groups = capt_group.split(split_char);

                            if !split_groups.any(|e| {
                                eprintln!("matching GROUP:{e}");
                                let res = match_by_char(&input_line[input_index..], e, true);
                                if res.0 {
                                    matched_input_len = res.1.unwrap();
                                }
                                res.0
                            }) {
                                eprintln!("\n\nSECOND SPLIT groups matching false for input:{input_line}, patt:{pattern}\n\n");
                                return (false, None);
                            }
                            input_index += matched_input_len;
                            eprintln!(
                                "AFTER MULTIPLE input index:{input_index}, patt_index:{patt_index}"
                            );
                        }
                        patt_index += capt_group.len() + 2;
                        prev_pattern = capt_group;
                    }
                }
                _ => {
                    if ['?', '$'].contains( &patt_chars[patt_index]) {
                        patt_index +=1;
                        continue;
                    }
                    prev_pattern = patt_chars[patt_index..patt_index + 1]
                        .into_iter()
                        .collect::<String>();
                    let is_optional =
                        patt_index + 1 < patt_len && patt_chars[patt_index + 1] == '?';
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
                            return (false, None);
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

    // if matching  partial return the length matched
    if full_match_optional && patt_index == pattern.len() {
        let ret_len = input_index;
        eprintln!(
            "BEFORE RETURN TRUE full optional:{full_match_optional}; input:{input_line},pattern{:?} returning an input length of:{}\n\n",
            pattern, ret_len
        );
        return (true, Some(ret_len));
    }

    if patt_index == patt_len {
        return (true, Some(patt_len));
    }
    // if input fully parsed but pattern not exhausted
    if input_index == input_chars.len() {
        //optional full parse
        //OR pattern fully parsed
        //OR pattern is optional/end marker
        //OR the remaining pattern is optional

        eprintln!("INPUT FULLY PARSED full optional:{full_match_optional}");
        if full_match_optional {
            return (true, Some(input_len));
        }
        let res = patt_index >= pattern.len()
            || ((patt_index == pattern.len() - 1)
                && ['$', '?', '.'].contains(&patt_chars[patt_index]))
            || patt_len - patt_index > 1 && {
                let remaining_patt = patt_chars[patt_index..].into_iter().collect::<String>();
                check_optional(&remaining_patt)
            };
        eprintln!("final return: input:{input_line}, i:{input_index}, input len:{input_len}\n  patt len:{patt_len}, pattern:{pattern}, patt i:{patt_index},\nres:{res}");
        return (res, Some(input_len));
    };
    eprintln!("\n\n:( COP OUT TRUE");
    eprintln!("input:{input_line}, pattern:{pattern}, input pos:{input_index}, patt_pos:{patt_index}");
    eprintln!("input len:{input_len}, pattern len:{patt_len}, ");
    return (false,None);
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
