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

    let res: Vec<usize> = res
        .iter()
        .map(|x| x.len())
        .inspect(|x| println!("2 {:?}", x))
        .collect::<Vec<usize>>();

    let res = res
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
        .sum::<usize>();

    println!("win: {:?}", res);
}

fn extract_num(v: &mut Vec<char>, idx: usize) -> String {
    let mut num: String = String::from("");

    let start = v[..idx].iter_mut().rev().map_while(|c| {
        if c.is_numeric() {
            let d = *c;
            *c = '.';
            return Some(d);
        }
        None
    });

    for c in start {
        num.insert(0, c)
    }

    let end = &mut v[idx..].iter_mut().map_while(|c| {
        if c.is_numeric() {
            let d = *c;
            *c = '.';
            return Some(d);
        }
        None
    });

    for c in end {
        num.push(c)
    }

    return num;
}
