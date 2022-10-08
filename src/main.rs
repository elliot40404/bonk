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

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    if (env::args().len() == 1 || env::args().nth(1).ok_or("Error")? == "-h")
        || (env::args().nth(1).ok_or("Error")? == "--help")
    {
        print_help();
    }
    let args = env::args_os()
        .map(PathBuf::from)
        .skip(1)
        .collect::<Vec<_>>();
    for arg in &args {
        if check_if_dir(arg.to_str().ok_or("Error")?) {
            if arg.to_str().ok_or("Error")?.ends_with('/')
                || arg.to_str().ok_or("Error")?.ends_with('\\')
            {
                fs::create_dir_all(arg)?;
            } else {
                fs::create_dir_all(arg.parent().ok_or("Error")?)?;
                if !arg.exists() {
                    File::create(arg)?;
                }
            }
        } else if !arg.exists() {
            File::create(arg)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(_e) = try_main() {
        println!("Something went wrong!");
        eprintln!("Error: {}", _e);
        std::process::exit(1);
    }
}
