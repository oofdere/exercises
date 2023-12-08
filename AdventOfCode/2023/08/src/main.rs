#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use fnv::FnvHashMap;
use rayon::prelude::*;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let lines = contents.lines();

    let inst = lines.clone().next().unwrap();

    println!("{:?}", inst);

    let res = lines
        .skip(2)
        .inspect(|x| println!("{x}"))
        .map(|x| x.split_once(" = ").unwrap())
        .map(|(idx, tup)| {
            let tup = &tup[1..tup.len() - 1].split_once(", ").unwrap();

            (idx, tup.clone())
        })
        .collect::<FnvHashMap<&str, (&str, &str)>>();

    println!("{:?}", res);

    let steps: usize = 0;
    let positions: Vec<usize> = res
        .iter()
        .map(|x| *x.0)
        .filter(|x| x.chars().last().unwrap() == 'A')
        .map(|x| {
            let mut steps = 0;
            let mut x = x;
            let mut inst = inst.clone().chars().cycle();

            while true {
                let i = inst.next().unwrap();

                let tup = res.get(x).unwrap();
                println!("{steps} {x} {tup:?}");
                let ne = match i {
                    'L' => tup.0,
                    'R' => tup.1,
                    _ => {
                        panic!("impossible!")
                    }
                };
                println!("{steps} {x} {ne:?}");
                steps += 1;
                x = ne;
                if x.chars().nth(2).unwrap() == 'Z' {
                    break;
                }
            }

            steps
        })
        .collect();

    println!("{:?}", positions);

    ()
}
