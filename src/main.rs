use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};

fn check_if_dir(dir: &str) -> bool {
    dir.contains("\\") || dir.contains("/")
}

fn print_help() {
    println!("
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
");
}

fn main() {
    if (env::args().nth(1).unwrap() == "-h") || (env::args().nth(1).unwrap() == "--help") {
        print_help();
        return;
    }
    let args = env::args_os()
        .map(|x| PathBuf::from(x))
        .skip(1)
        .collect::<Vec<_>>();
    for arg in &args {
        if check_if_dir(arg.to_str().unwrap()) {
            if arg.to_str().unwrap().ends_with("/") || arg.to_str().unwrap().ends_with("\\") {
                fs::create_dir_all(arg).expect("Failed to create directory");
            } else {
                fs::create_dir_all(arg.parent().unwrap()).expect("Failed to create directory");
                File::create(arg.to_owned()).expect("failed to create file");
            }
        } else {
            File::create(arg.to_owned()).expect("failed to create file");
        }
    }
}
