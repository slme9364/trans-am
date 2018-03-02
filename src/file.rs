use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::env;
use std::vec::Vec;

pub fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return "cache".to_owned();
    }
    args[1].clone()
}

pub fn get_file_line_no() -> usize {
    let text = open_file();
    text.len()
}

pub fn open_file() -> Vec<String> {
    let file_path = get_file_name();
    let text = match file_read(file_path.as_str()) {
        Some(data) => data,
        None => {
            let mut vec = Vec::new();
            vec.push("".to_owned());
            return vec;
        }
    };
    let str_vec: Vec<&str> = text.as_str().split('\n').collect();
    let text_vec: Vec<String> = str_vec.iter().map(|&s| s.to_owned()).collect();
    text_vec
}

pub fn file_read(file_name: &str) -> Option<String> {
    let file_path = Path::new(file_name);

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut text_data: String = String::new();
    file.read_to_string(&mut text_data);
    Some(text_data)
}
