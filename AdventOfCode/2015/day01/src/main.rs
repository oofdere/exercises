use std::{cmp::Ordering, env, fs};

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
        .enumerate()
        .fold((None, 0), |acc, (i, e)| {
            let floor = acc.1 + e;
            println!("{i} {:?}", acc);
            if acc.0 == None && floor < 0 {
                (Some(i + 1), floor)
            } else {
                (acc.0, floor)
            }
        });

    println!("The star is on floor {}!", floor.1);

    match floor.0 {
        Some(n) => println!("Santa first enters the basement in move {n}"),
        None => println!("Santa does not enter the basement."),
    }
}
