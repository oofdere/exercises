use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let lns = contents.lines();

    let former = lns
        .clone()
        .map(|l| {
            let lin: String = String::from(l);
            let mut ms: Vec<(usize, &str)> = vec![];
            for word in numbers {
                let mut m: Vec<(usize, &str)> = lin.match_indices(word).collect();

                ms.append(&mut m);
            }
            ms.sort_by(|(i, _), (j, _)| i.cmp(j));

            let ms = ms.first();
            let ms = match ms {
                Some(e) => e,
                None => {
                    println!("Hit none on {lin}");
                    return lin;
                }
            };
            let mut lin = lin.clone();

            lin.replace_range(
                ms.0..ms.0 + ms.1.len(),
                &numbers.iter().position(|n| n == &ms.1).unwrap().to_string(),
            );

            lin
        })
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>());

    let latter = lns
        .map(|l| {
            let lin: String = String::from(l);
            let mut ms: Vec<(usize, &str)> = vec![];
            for word in numbers {
                let mut m: Vec<(usize, &str)> = lin.match_indices(word).collect();

                ms.append(&mut m);
            }
            ms.sort_by(|(i, _), (j, _)| i.cmp(j));

            let ms = ms.last();
            let ms = match ms {
                Some(e) => e,
                None => {
                    println!("Hit none on {lin}");
                    return lin;
                }
            };
            let mut lin = lin.clone();

            lin.replace_range(
                ms.0..ms.0 + ms.1.len(),
                &numbers.iter().position(|n| n == &ms.1).unwrap().to_string(),
            );

            lin
        })
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>());

    let res: i32 = former
        .zip(latter)
        .inspect(|f| println!("2 {:?}", f))
        .map(|(l, f)| format!("{}{}", l[0], f.last().unwrap()))
        .inspect(|f| println!("3 {:?}", f))
        .map(|n| n.parse::<i32>().unwrap())
        .inspect(|f| println!("4 {:?}", f))
        .sum();

    println!("{:?}", res)
}
