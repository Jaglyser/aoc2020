use std::collections::HashSet;

fn main() {
    let mut hash_set: HashSet<i32> = HashSet::new();
    let values: Vec<i32> = vec![1721, 979, 366, 299, 675, 1456];
    let win = 2020;

    for &value in values.iter() {
        let temp: i32 = win - value;
        if hash_set.contains(&temp) {
            println!("You win! Product: {:?}", temp * value)
        }
        hash_set.insert(value);
    }
}
