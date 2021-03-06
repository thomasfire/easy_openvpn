use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::fs::{read_dir, File};
use std::path::Path;

/// Reads filename and returns String
///
/// Reads filename and returns String, with replaced CRLF to LF
///
/// # Examples
///
/// ```rust
/// let file_string = read_str("path/to/file");
/// ```
pub fn read_str(filename: &str) -> String {
    let mut f = File::open(filename).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Couldn`t read to string");
    String::from(buffer.replace("\r\n", "\n").trim())
}

/// Reads line from stdin and returns trimed String
///
/// Reads line from stdin and returns trimed String.
/// It is similar to `input()` in Python, which returns striped string
///
/// # Examples
/// ```rust
/// let some_useful_string = read_std_line();
/// ```
pub fn read_std_line(output: &str) -> String {
    let mut buffer = String::new();
    print!("{}", output);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn`t read std");
    String::from(buffer.trim())
}

/// Returns true if path exists and false if not
pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Writes String to your file
///
/// # Examples
///
/// ```rust
/// write_to_file("/path/to/file", "I`m file");
/// ```
pub fn write_to_file(path: &str, content: String) -> Result<(), io::Error> {
    let mut file = File::create(path).unwrap();
    file.write_fmt(format_args!("{}", content))
}

/// Returns only *.ovpn files of directory
///
/// # Examples
///
/// ```rust
/// let ovpn_files: Vec<&str> = get_ovpn_files("path/to/dir");
/// ```
pub fn get_ovpn_files(path: &str) -> Vec<String> {
    if !exists(path) {
        return vec![];
    }
    let entries = read_dir(path).unwrap();
    let mut files: Vec<String> = vec![];
    for entry in entries {
        let smentry = entry.unwrap().path();
        let en_path = smentry.extension();
        if en_path == None {
            continue;
        }
        if smentry.is_file() && en_path.unwrap() == "ovpn" {
            files.push(String::from(smentry.file_name().unwrap().to_str().unwrap()));
        }
    }

    files
}
