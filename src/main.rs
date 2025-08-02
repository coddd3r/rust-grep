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
    let patt_letters: Vec<char> = pattern.chars().collect();
    let pat_chars = pattern.chars();
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

    let res = {
        if pattern.chars().nth(0).unwrap() == '[' && pat_chars.clone().last().unwrap() == ']' {
            eprintln!("FOUND char group");

            let lett_group = &patt_letters[2..patt_letters.len() - 1];
            pattern.len() > 2 && {
                if patt_letters[1] != '^' {
                    input_line
                        .chars()
                        .into_iter()
                        .any(|c| lett_group.contains(&c))
                } else {
                    eprintln!("checking negative group");

                    input_line
                        .chars()
                        .into_iter()
                        .any(|c| !lett_group.contains(&c))
                }
            }
        } else {
            match pattern.as_str() {
                r"\d" => input_line.chars().into_iter().any(|e| e.is_digit(10)),
                r"\w" => input_line
                    .chars()
                    .into_iter()
                    .any(|e| e.is_alphanumeric() || e == '_'),
                _ => match_pattern(&input_line, &pattern),
            }
        }
    };

    if res {
        eprintln!("SUCCESS");
        process::exit(0)
    } else {
        eprintln!("FAILED");
        process::exit(1)
    }
}
