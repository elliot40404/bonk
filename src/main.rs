use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};

fn check_if_dir(dir: &str) -> bool {
    dir.contains('\\') || dir.contains('/')
}

fn print_help() {
    println!(
        "
Description:
  A blazingly fast alternative to the classic 'touch' command.
  Can create a single file or multiple files even in nested directories.
Options: 
-h, --help: Show this help message
Example:
  bonk bonkers.txt
  bonk bonkers.txt bonkers2.txt
  bonk bonky/chonky/boi.txt
  bonk bonky/chonky/ bonkers.txt
"
    );
}

fn main() {
    if (env::args().len() == 1 || env::args().nth(1).unwrap() == "-h")
        || (env::args().nth(1).unwrap() == "--help")
    {
        print_help();
        return;
    }
    let args = env::args_os()
        .map(PathBuf::from)
        .skip(1)
        .collect::<Vec<_>>();
    for arg in &args {
        if check_if_dir(arg.to_str().unwrap()) {
            if arg.to_str().unwrap().ends_with('/') || arg.to_str().unwrap().ends_with('\\') {
                match fs::create_dir_all(arg) {
                    Ok(_) => {}
                    Err(_) => {
                        println!(
                            "Error: Could not create directory '{}'",
                            arg.to_str().unwrap()
                        );
                        std::process::exit(1);
                    }
                }
            } else {
                match fs::create_dir_all(arg.parent().unwrap()) {
                    Ok(_) => {}
                    Err(_) => {
                        println!(
                            "Error: Could not create directory '{}'",
                            arg.parent().unwrap().to_str().unwrap()
                        );
                        std::process::exit(1);
                    }
                }
                if !arg.exists() {
                    match File::create(arg) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("Error: Could not create file '{}'", arg.to_str().unwrap());
                            std::process::exit(1);
                        }
                    }
                }
            }
        } else if !arg.exists() {
            match File::create(arg) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error: Could not create file '{}'", arg.to_str().unwrap());
                    std::process::exit(1);
                }
            }
        }
    }
}
