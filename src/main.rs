use std::{env, fs, process, collections::HashMap};
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough directories specified");
        process::exit(1);
    }
    let mut files: HashMap<String, u64> = HashMap::new();
    match fs::read_dir(&args[1]) {
        Ok(dir_content) => {
            for file in dir_content {
                match file {
                    Ok(file) => {
                            match file.file_name().to_str() {
                                Some(path) => {
                                    let _ = files.insert(path.to_string(), file.metadata().unwrap().len());
                                },
                                None => (),
                            }
                        }
                    Err(_) => (),
                }
            }
        },
        Err(_) => {
            println!("Directory {} could not be read", &args[1]);
            process::exit(1);
        }
    }
    println!("{0: <20} | {1: <10} | {2: <10}", "file", "old size", "new size");
    match fs::read_dir(&args[2]) {
        Ok(dir_content) => {
            for file in dir_content {
                match file {
                    Ok(file) => {
                        match file.file_name().to_str() {
                            Some(path) => {
                                let old: u64 = *files.entry(path.to_string()).or_default();
                                let new = file.metadata().unwrap().len();
                                if old == 0 {
                                    let old_size = format!("{}", old);
                                    let new_size = format!("{}", new);
                                    println!("{0: <20} | {1: <10} | {2: <10}", path, old_size, new_size);
                                }
                                else if old > new {
                                    let old_size = format!("{}", old).red();
                                    let new_size = format!("{}", new).green();
                                    println!("{0: <20} | {1: <10} | {2: <10}", path, old_size, new_size);
                                }
                                else {
                                    let old_size = format!("{}", old).green();
                                    let new_size = format!("{}", new).red();
                                    println!("{0: <20} | {1: <10} | {2: <10}", path, old_size, new_size);
                                }
                            },
                            None => (),
                        }
                    }
                    Err(_) => (),
                }
            }
        },
        Err(_) => {
            println!("Directory {} could not be read", &args[2]);
            process::exit(1);
        }
    }
}
