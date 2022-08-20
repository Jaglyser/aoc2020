use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_input(path: String) -> Vec<i32> {
    let file: File = File::open(Path::new(&path)).expect("file wasn't found.");
    let reader: BufReader<File> = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    return numbers;
}
