use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use clap::{App, Arg};

fn main() {
    let matches = App::new("texcount")
        .author("Kai Schmidt <kaikaliischmidt@gmail.com>")
        .about("Count the total number of words in every .tex file in the current directory")
        .arg(Arg::with_name("INPUT"))
        .get_matches();
    let master_path = PathBuf::from(matches.value_of("INPUT").unwrap_or("."));
    match count_dir(master_path) {
        Ok(count) => println!("{} words", count),
        Err(e) => println!("Error: {}", e),
    }
}

fn count_dir<P: AsRef<Path>>(path: P) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    for entry in fs::read_dir(path)?.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            count += count_dir(path)?;
        } else if let Some(extension) = path.extension() {
            if extension == "tex" {
                count += count_file(path)?;
            }
        }
    }
    Ok(count)
}

fn count_file<P: AsRef<Path>>(path: P) -> Result<usize, Box<dyn Error>> {
    let file_string = fs::read_to_string(path)?;
    Ok(file_string
        .split_whitespace()
        .filter(|word| !word.contains('\\'))
        .count())
}
