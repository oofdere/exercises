use std::{cmp::Ordering, collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let lines = contents.lines();

    let res = lines
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    println!("{:?}", res);

    let res = res
        .clone()
        .into_iter()
        .map(|x| {
            let mut vec = vec![x];

            let mut sum = 64;
            while sum != 0 {
                let w = vec
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|y| y[1] - y[0])
                    .collect::<Vec<i64>>();

                sum = w.iter().sum::<i64>() + vec.last().unwrap().iter().sum::<i64>();

                vec.push(w);
            }

            println!("{vec:?}");

            let vec = vec
                .iter()
                .map(|x| *x.first().unwrap())
                .collect::<Vec<i64>>();

            vec
        })
        .collect::<Vec<Vec<i64>>>();

    println!("{:?}", res);

    let res = res
        .iter()
        .map(|seq| {
            let mut prev = 0;
            seq.iter()
                .rev()
                .map(|x| {
                    let y = x - prev;
                    println!("{x} - {prev} = {y}");
                    prev = y;
                    y
                })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    println!("{:?}", res);

    let res = res.iter().map(|x| *x.last().unwrap()).collect::<Vec<i64>>();
    println!("aaaaa {:?}", res);

    let res = res.iter().sum::<i64>();

    println!("{:?}", res);

    ()
}
