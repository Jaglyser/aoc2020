use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let mut hash_set: HashSet<i32> = HashSet::new();
    // let values: Vec<i32> = vec![1721, 979, 366, 299, 675, 1456];
    let values = read_input();
    let win = 2020;

    for &value in values.iter() {
        let temp: i32 = win - value;
        if hash_set.contains(&temp) {
            println!("You win! Product: {:?}", temp * value)
        }
        hash_set.insert(value);
    }
}

fn read_input() -> Vec<i32> {
    let file = File::open(Path::new("resources/1.txt")).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    return numbers;
}
