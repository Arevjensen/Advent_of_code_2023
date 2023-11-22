use std::{
    fs,
    io::{self, BufRead},
};

pub fn read_lines_from_file(path: &str) -> Vec<String> {
    let file = fs::File::open(path).unwrap();

    io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect()
}

pub fn get_input_path(part: &str, day: &str) -> String {
    format!("input/day_{day}_{part}.txt")
}

pub fn read_text_from_file(part: &str, day: &str) -> String {
    let path = get_input_path(part, day);

    fs::read_to_string(path).unwrap()
}
