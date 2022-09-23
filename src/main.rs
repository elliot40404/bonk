use std::fs::File;
use std::path::Path;
use std::{env, fs};

fn check_if_dir(dir: &str) -> bool {
    if dir.contains('/') || dir.contains('\\') {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut input: Vec<String> = env::args().collect();
    input.remove(0);
    input.iter().for_each(|x| {
        if check_if_dir(x) {
            let sep = if x.contains('/') { '/' } else { '\\' };
            let mut dir = x.split(sep).collect::<Vec<&str>>();
            if Path::new(dir[0]).exists() {
                return println!("{}: A file with same name already exists", dir[0]);
            }
            dir.pop();
            let dir = dir.join(&sep.to_string());
            fs::create_dir_all(&dir).expect("Directory already exists");
            if !Path::new(x).exists() {
                File::create(String::from(x)).expect("Unable to create file in directory");
            }
        } else {
            if !Path::new(x).exists() {
                File::create(String::from(x)).expect("Unable to create file");
            }
        }
    });
}
