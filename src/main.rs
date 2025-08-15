use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::process;

use codecrafters_grep::match_by_char;

fn main() {
    eprintln!("Logs from your program will appear here!");

    let mut all_args = env::args();
    let len_args = all_args.len();
    eprintln!("args:{:?}", all_args);
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let mut res = false;
    let pattern = env::args().nth(2).unwrap();

    if len_args > 3 {
        for _ in 0..3 {
            let _ = all_args.next();
        }

        while let Some(input_file) = &all_args.next() {
            eprintln!("\n\n=====checking file:{input_file}=======");
            let input_file = std::env::current_dir().unwrap().join(&input_file);
            if input_file.exists() {
                let file = File::open(input_file).unwrap();
                let reader = BufReader::new(file);

                reader.lines().for_each(|l| {
                    if let Ok(input_line) = l {
                        eprintln!("\n~~~~~~for line:{input_line}");
                        let curr_res = match_by_char(&input_line, &pattern, false, &Vec::new()).0;
                        if curr_res {
                            println!("{input_line}");
                        }
                        res = res || curr_res;
                    }
                });
            }
        }
    } else {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        res = match_by_char(&input_line, &pattern, false, &Vec::new()).0;
    }

    if res {
        eprintln!("SUCCESS");
        process::exit(0)
    } else {
        eprintln!("FAILED");
        process::exit(1)
    }
}
