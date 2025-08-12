use std::env;
use std::io;
use std::process;

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    eprintln!("Logs from your program will appear here!");

    let all_args = env::args();
    eprintln!("args:{:?}", all_args);
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();
    // if pattern.chars().nth(0).unwrap() == '[' && pat_chars.clone().last().unwrap() == ']' {
    //     eprintln!("FOUND char group");
    //     if input_line.chars().into_iter().any(|c| {
    //         patt_letters[1..patt_letters.len() - 1]
    //             .iter()
    //             .any(|d| &c == d)
    //     }) {
    //         eprintln!("char group SUCCESS");
    //         process::exit(0)
    //     } else {
    //         eprintln!("char group FAILED");
    //         process::exit(1)
    //     }
    // }

    // let res = {
    //     if pattern.chars().nth(0).unwrap() == '[' && pat_chars.clone().last().unwrap() == ']' {
    //         eprintln!("FOUND char group");

    //         let lett_group = &patt_letters[2..patt_letters.len() - 1];
    //         pattern.len() > 2 && {
    //             if patt_letters[1] != '^' {
    //                 input_line
    //                     .chars()
    //                     .into_iter()
    //                     .any(|c| lett_group.contains(&c))
    //             } else {
    //                 eprintln!("checking negative group");

    //                 input_line
    //                     .chars()
    //                     .into_iter()
    //                     .any(|c| !lett_group.contains(&c))
    //             }
    //         }
    //     } else {
    //         if pattern.contains(r"\d") {
    //             pattern.match_indices(r"\d").all(|(i, _)| {
    //                 // get corresponding index to check if it is a digit
    //                 input_line.chars().nth(i).unwrap().is_digit(10)
    //             })
    //         } else {
    //             match pattern.as_str() {
    //                 r"\d" => input_line.chars().into_iter().any(|e| e.is_digit(10)),
    //                 r"\w" => input_line
    //                     .chars()
    //                     .into_iter()
    //                     .any(|e| e.is_alphanumeric() || e == '_'),
    //                 _ => match_pattern(&input_line, &pattern),
    //             }
    //         }
    //     }
    // };

    let res = match_by_char(&input_line, &pattern);
    if res {
        eprintln!("SUCCESS");
        process::exit(0)
    } else {
        eprintln!("FAILED");
        process::exit(1)
    }
}

fn match_by_char(input_line: &str, pattern: &str) -> bool {
    let patt_chars: Vec<char> = pattern.chars().collect();
    let input_chars: Vec<char> = input_line.chars().collect();
    let mut patt_index: usize = 0;
    let mut input_index = 0;
    let patt_len = pattern.len();
    let input_len = input_line.len();

    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        while patt_index < patt_len && input_index < input_len {
            // eprintln!("start of while input i:{input_index}, pattern i:{patt_index}");
            if patt_chars[patt_index] == '[' && patt_chars[patt_index + 1..].contains(&']') {
                let char_group_end = patt_chars[patt_index + 1..]
                    .iter()
                    .position(|c| c == &']')
                    .unwrap();

                let char_group_length = char_group_end - patt_index - 1;
                let lett_group = &patt_chars[patt_index + 1..patt_index + char_group_length];

                if !(char_group_length > 1 && {
                    if patt_chars[patt_index + 1] != '^' {
                        input_chars[input_index..input_index + char_group_length]
                            .iter()
                            .any(|c| lett_group.contains(&c))
                    } else {
                        eprintln!("checking negative group");

                        input_line
                            .chars()
                            .into_iter()
                            .any(|c| !lett_group.contains(&c))
                    }
                }) {
                    return false;
                }
                patt_index = char_group_end;
            } else if &pattern[patt_index..patt_index + 1] == r"\" {
                while &pattern[patt_index + 1..patt_index + 2] == r"\" {
                    patt_index += 1;
                }
                let char_class = &pattern[patt_index..patt_index + 2];
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
            } else {
                if &pattern[patt_index..patt_index + 1] != &input_line[input_index..input_index + 1]
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

    // if input fully parsed but pattern not exhausted
    if input_index == input_chars.len() {
        eprintln!("final return: input i:{input_index}, patt i:{patt_index}");
        return patt_index >= pattern.len();
    };
    true
}
