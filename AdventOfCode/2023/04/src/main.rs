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
    let res = res
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
            win.len()
        })
        .collect::<Vec<usize>>();

    println!("{:?}", res);

    let mut a = vec![0; res.len() + 100];

    for (idx, c) in res.iter().enumerate().rev() {
        // for each card, add the value in idx[a]
        a[idx] += 1;
        for i in idx + 1..idx + c + 1 {
            println!("loop!{i}");
            a[idx] += a[i]
        }
        println!("({idx}, {c}), {:?}", &a[..res.len()]);
    }

    let a: usize = a.iter().sum::<usize>();

    dbg!(a);
}
