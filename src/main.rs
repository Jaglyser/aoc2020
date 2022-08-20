mod read_file_aoc;
use std::collections::HashSet;

fn main() {
    let p1: i32 = part_one();
    let p2: i32 = part_two();

    println!("First product: {:?} \nSecond Product: {:?}", p1, p2);
}

fn part_one() -> i32 {
    let mut hash_set: HashSet<i32> = HashSet::new();
    let values: Vec<i32> = read_file_aoc::read_input("resources/1.txt".to_string());
    let win: i32 = 2020;

    for &first_val in values.iter() {
        let second_val: i32 = win - first_val;
        if hash_set.contains(&second_val) {
            // println!("You win! Product: {:?}", temp * value)
            return first_val * second_val;
        }
        hash_set.insert(first_val);
    }
    return 0;
}

fn part_two() -> i32 {
    let mut hash_set: HashSet<i32> = HashSet::new();
    let values: Vec<i32> = read_file_aoc::read_input("resources/1.txt".to_string());
    let win: i32 = 2020;

    for &first_val in values.iter() {
        let current_sum: i32 = win - first_val;
        for &second_val in values.iter() {
            let third_val: i32 = current_sum - second_val;
            if hash_set.contains(&third_val) {
                return first_val * second_val * third_val;
            }
            hash_set.insert(first_val);
        }
    }
    return 0;
}
