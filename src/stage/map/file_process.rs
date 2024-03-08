use std::fs;
use std::io::Error;

use crate::stage::map::map_constants::*;

pub const MAP_TXT_PATH: &str = "./assets";
pub const MAP_TXT_FILE: &str = "./maptxt/map.txt";

pub fn get_txt_files(dir: &str) -> Vec<String> {
    let mut txt_files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if ext == "txt" {
                        if let Some(file_name) = entry.path().file_name() {
                            if let Some(file_name) = file_name.to_str() {
                                txt_files.push(file_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    txt_files
}

pub fn map_txt_init(path: &str) {
    let arr = vec![vec!['_' ; MAP_COLUMNS]; MAP_ROWS];
    map_txt_write(path, &arr);  
}

pub fn map_txt_modify(array: &mut Vec<Vec<char>>,index: (usize, usize), value: char) {
    array[index.0][index.1] = value;
}



pub fn map_txt_write(path: &str, map: &Vec<Vec<char>>) {
    let arr_string = map.iter()
    .map(|row| row.iter().map(|&i| i.to_string()).collect::<Vec<_>>().join(" "))
    .collect::<Vec<_>>()
    .join("\n");

    fs::write(path, arr_string).expect("Unable to write file");
}

pub fn map_txt_read(path: &str) -> Result<Vec<Vec<char>>, Error> {
    let file_content = fs::read_to_string(path)?;
    let map: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.split_whitespace().map(|c| c.chars().next().unwrap()).collect())
        .collect();

    Ok(map)
}