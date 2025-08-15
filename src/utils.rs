use std::io::BufRead;
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::match_by_char;

pub fn check_optional(pattern: &str) -> bool {
    let mut patt_index: usize = 0;
    let patt_len = pattern.len();
    let patt_chars: Vec<char> = pattern.chars().collect();
    let optional_markers = ['*', '?'];
    match patt_chars[patt_index] {
        '[' => {
            if patt_chars[patt_index + 1..].contains(&']') {
                let char_group_end = patt_chars[patt_index + 1..]
                    .iter()
                    .position(|c| c == &']')
                    .unwrap();
                let optional_position = patt_index + char_group_end + 2;
                optional_position < patt_len
                    && optional_markers.contains(&patt_chars[optional_position])
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
                patt_len > closing_bracket_index
                    && optional_markers.contains(&patt_chars[closing_bracket_index])
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
            patt_index + 2 < patt_len && optional_markers.contains(&patt_chars[patt_index + 2])
        }

        _ => {
            if ['?'].contains(&patt_chars[patt_index]) {
                patt_index += 1;
            }
            let opt_position = patt_chars[patt_index + 1];
            eprintln!("opt position:{opt_position}");
            patt_index + 1 < patt_len && optional_markers.contains(&opt_position)
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

        if next_pattern == "$" {
            break;
        }
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

pub fn get_numrepeats(
    input_index: usize,
    input_line: &str,
    use_patt: String,
    input_chars: &Vec<char>,
    patt_capture_groups: &Vec<(usize, usize, String)>,
) -> (usize, usize) {
    let mut num_repeats = 0;
    let input_len = input_line.len();
    let mut matched_size = 0;
    let mut use_i = input_index;
    while use_i < input_len {
        let use_input = &input_chars[use_i..].iter().collect::<String>();
        eprintln!("\n\ncalling repeat,input chars:{:?} input i:{use_i} using input:{:?}, input length{:?}",input_chars, use_input, input_len);
        let res = match_by_char(&use_input, &use_patt, true, &patt_capture_groups);
        if !res.0 {
            break;
        }
        eprintln!("in loop res?:{:?}", res);
        eprintln!("in loop input i?:{:?}", use_i);
        matched_size += res.1.unwrap();
        use_i += res.1.unwrap();
        num_repeats += 1;
    }
    (num_repeats, matched_size)
}

pub fn match_extra(
    prev_pattern: &str,
    input_chars: &Vec<char>,
    input_line: &str,
    input_index: usize,
    patt_capture_groups: &Vec<(usize, usize, String)>,
    patt_index: usize,
    patt_chars: &Vec<char>,
) -> (bool, usize) {
    eprintln!("in MATCH EXTRA");
    // if there are more of the same immediately after e.g "ca+ats" for "caaaats"
    // move pattern pointer forward by one
    // move the input index forward by at least 1
    // or as many as possible while still satisfying the rest of the pattern
    let mut use_i = input_index;
    //full number of times the prev matched can occur
    let use_patt = format!("^{prev_pattern}");
    let input_len = input_chars.len();
    let rem_patt: String = patt_chars[patt_index + 1..].iter().collect();

    //if it ends after ths repeat
    //make sure patt matches to end
    //if patt has free reight, match as much as possible
    if rem_patt.len() == 1 && &rem_patt == "$" || rem_patt.is_empty() {
        eprintln!("checking END OR LAST patt");
        let (_num_repeats, matched_size) = get_numrepeats(
            use_i,
            input_line,
            use_patt,
            &input_chars,
            &patt_capture_groups,
        );
        use_i += matched_size;
        let res = (rem_patt.is_empty() || use_i == input_len, use_i);
        eprintln!("RETURNING FROM EXTRA CHECKER:{res:?}\n");
        return res;
    }
    let mut check_index = input_len - 1;
    while check_index > input_index {
        eprintln!("in match extra while");
        let curr_input: String = input_chars[check_index..].iter().collect();
        if match_by_char(&curr_input, &rem_patt, false, patt_capture_groups).0 {
            break;
        }
        check_index -= 1;
    }
    eprintln!("FOUND PATT AT INDEX:{check_index}");
    eprintln!("input i:{input_index}");
    let res = (check_index >= input_index, check_index);
    eprintln!("RETURNING FROM EXTRA CHECKER:{res:?}\n");
    res
}

pub fn get_paths(dir: PathBuf) -> Vec<PathBuf> {
    let mut all_paths: Vec<PathBuf> = Vec::new();

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).unwrap() {
            if let Ok(ent) = entry {
                let path = ent.path();
                if path.is_dir() {
                    all_paths.extend(get_paths(path));
                } else {
                    all_paths.push(path);
                }
            }
        }
    }

    all_paths
}

pub fn parse_file(f: &PathBuf, multiple_files: bool, pattern: &str) -> bool {
    let mut res = false;
    let input_file = &f;
    if input_file.exists() {
        //println!("checking file:{input_file:?}");
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);

        reader.lines().for_each(|l| {
            if let Ok(input_line) = l {
                //println!("checking line:{input_line}");
                eprintln!("\n~~~~~~for line:{input_line}");
                let curr_res = match_by_char(&input_line, &pattern, false, &Vec::new()).0;
                if curr_res {
                    if multiple_files {
                        print!("{:?}:", f);
                    }
                    println!("{input_line}");
                }
                res = res || curr_res;
            }
        });
    }
    res
}
