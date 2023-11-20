use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let floor = contents
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("The star is on floor {floor}!")
}
