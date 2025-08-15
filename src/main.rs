use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process;
use std::{env, fs};

use codecrafters_grep::match_by_char;

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

fn get_paths(dir: PathBuf) -> Vec<PathBuf> {
    let mut all_paths: Vec<PathBuf> = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
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

fn parse_file(f: &PathBuf, multiple_files: bool, pattern: &str) -> bool {
    let mut res = false;
    let input_file = &f;
    if input_file.exists() {
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);

        reader.lines().for_each(|l| {
            if let Ok(input_line) = l {
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
    println!("for file:{f:?} returning:{res}");
    res
}
