use std::{cell::UnsafeCell, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    #[derive(Debug)]
    struct Pull(i32, i32, i32);

    let lines = contents.lines();

    // remove game nums
    let res = lines.map(|x| x.split(": ").last().unwrap());

    // split into two things
    let res: Vec<Vec<i32>> = res
        .map(|x| x.split(" | ").collect::<Vec<&str>>())
        .map(|x| {
            x.into_iter()
                .map(|y| y.split_whitespace().collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        })
        .inspect(|x| println!("0 {x:?}"))
        .map(|x| {
            x.into_iter()
                .map(|y| {
                    y.into_iter()
                        .map(|z| z.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .inspect(|x| println!("1 {x:?}"))
        .map(|x| {
            let mut win: Vec<i32> = vec![];

            for i in &x[0] {
                for j in &x[1] {
                    if i == j {
                        win.push(*j)
                    }
                }
            }

            win
        })
        .collect::<Vec<Vec<i32>>>();

    // contains all the wins from the cards
    let counts: Vec<usize> = res
        .iter()
        .map(|x| x.len())
        .inspect(|x| println!("2 {:?}", x))
        .collect::<Vec<usize>>();

    let scores = counts
        .iter()
        .map(|x| {
            if *x == 0 {
                return *x;
            };
            let mut mul = 1;

            for _ in 1..*x {
                mul *= 2;
            }

            mul
        })
        .collect::<Vec<usize>>();
    println!("counts: {:?}", counts);
    println!("scores: {:?}", scores);

    // disgusting
    let counts = [counts, vec![0; 100]].concat();

    let res: usize = counts
        .iter()
        .enumerate()
        .map(|(idx, w)| {
            let winnas = &scores[idx..idx + w];

            dbg!(winnas);

            winnas.iter().sum::<usize>()
        })
        .sum();

    dbg!(res);
}
