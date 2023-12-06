use std::{cell::UnsafeCell, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    #[derive(Debug)]
    struct Pull(usize, usize, usize);

    let lines = contents.lines();

    let mut res = lines.map(|x| {
        // get rid of t/d
        let x = x.split_once(":").unwrap().1;
        let x = x.split_whitespace().collect::<String>();
        println!("{x}");
        let x = vec![x.parse::<usize>().unwrap()];

        x
    });

    let times = res.next().unwrap();
    let distances = res.next().unwrap();

    // combine into one array
    let res = times
        .iter()
        .zip(distances.iter())
        .collect::<Vec<(&usize, &usize)>>();
    // each ms held increases speed by 1mm/sec

    println!("{:?}", res);

    // find the ranges of winning rumbers
    // and multiply in one fell swoop

    let res = res
        .iter()
        .map(|(time, dist)| {
            // number of ways to win
            let mut wins = 0;
            for i in 1..**time {
                // remove the time elapsed pressing button
                let left = *time - i;

                // multiply time left by speed to get distance
                let d = i * left;

                // if win add to wins
                if d > **dist {
                    wins += 1;
                }
            }

            wins
        })
        .collect::<Vec<usize>>();

    println!("{:?}", res);

    let res = res.into_iter().reduce(|acc, e| acc * e);

    println!("{:?}", res.unwrap());

    ()
}
