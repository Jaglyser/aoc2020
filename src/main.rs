mod read_file_aoc;
use std::collections::HashSet;

fn main() {
    let mut hash_set: HashSet<i32> = HashSet::new();
    let values: Vec<i32> = read_file_aoc::read_input("resources/1.txt".to_string());
    let win: i32 = 2020;

    for &value in values.iter() {
        let temp: i32 = win - value;
        if hash_set.contains(&temp) {
            println!("You win! Product: {:?}", temp * value)
        }
        hash_set.insert(value);
    }
    part_two();
}

fn part_two() {
    let mut hash_set: HashSet<i32> = HashSet::new();
    let values: Vec<i32> = read_file_aoc::read_input("resources/1.txt".to_string());
    let win: i32 = 2020;

    for &value in values.iter() {
        let current_sum: i32 = win - value;
        for &j in values.iter() {
            let temp: i32 = current_sum - j;
            if hash_set.contains(&temp) {
                println!("You win! Product: {:?}", j * value * temp)
            }
            hash_set.insert(value);
        }
    }
}
