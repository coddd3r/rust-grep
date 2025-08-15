use std::env;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::process;

use codecrafters_grep::match_by_char;
use codecrafters_grep::utils::{get_paths, parse_file};

fn main() {
    eprintln!("Logs from your program will appear here!");

    let mut all_args = env::args();
    let len_args = all_args.len();
    eprintln!("args:{:?}", all_args);
    if env::args().nth(2).unwrap() != "-E" && env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let mut res = false;
    let pattern = env::args().nth(2).unwrap();

    if len_args > 3 {
        eprintln!("file args");
        let multiple_files = len_args > 4;

        let mut all_paths: Vec<PathBuf> = Vec::new();
        if env::args().any(|a| &a == "-r") {
            let z = env::args().nth(4).unwrap();
            let first_path = Path::new(&z);
            eprintln!("firs path:{:?}", first_path);
            if first_path.metadata().unwrap().is_dir() {
                all_paths = get_paths(first_path.to_path_buf());
            }
        } else {
            let _ = all_args.nth(2); //consume first 3
            eprintln!("before while all args:{:?}", all_args);
            while let Some(f) = &all_args.next() {
                eprintln!("\n\n=====checking file:{f}=======");
                let p = std::env::current_dir().unwrap().join(&f);
                all_paths.push(p);
            }
        }

        all_paths
            .iter()
            .for_each(|p| res = res || parse_file(p, multiple_files, &pattern));
        eprintln!("ALL FILES:{all_paths:?}")
    } else {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        res = match_by_char(&input_line, &pattern, false, &Vec::new()).0;
    }

    if res {
        eprintln!("SUCCESS!!!");
        process::exit(0)
    } else {
        eprintln!("FAILED");
        process::exit(1)
    }
}
