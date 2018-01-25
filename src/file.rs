use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::env;
use std::vec::Vec;

pub fn open_file() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        let mut rval: Vec<String> = Vec::new();
        rval.push("".to_owned());
        return rval;
    }
    let file_path = args[1].as_str();

    let text = match file_read(file_path) {
        Some(data) => data,
        None => return Vec::new(),
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
