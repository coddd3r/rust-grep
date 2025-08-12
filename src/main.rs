use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

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

    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        let mut patt_index: usize = 0;
        let mut input_index = 0;
        let patt_len = pattern.len();
        let input_len = input_line.len();

        while patt_index < patt_len && input_index < input_len {
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
                let curr_remaining = &input_line[input_index..];
                if !(match &pattern[patt_index..patt_index + 2] {
                    r"\d" => curr_remaining.chars().into_iter().any(|e| e.is_digit(10)),
                    r"\w" => curr_remaining
                        .chars()
                        .into_iter()
                        .any(|e| e.is_alphanumeric() || e == '_'),
                    _ => match_pattern(&curr_remaining, &pattern[patt_index..]),
                }) {
                    return false;
                }
            }
            patt_index += 1;
            input_index += 1;
        }
    }
    true
}
