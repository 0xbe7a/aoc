use std::env;
use std::fs;

pub mod solutions;

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(folder)
        .join(format!("day{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

pub fn read_file_with_name(folder: &str, name: &str) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(folder)
        .join(format!("day{}.txt", name));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}