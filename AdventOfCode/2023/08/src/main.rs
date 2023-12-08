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

    let mut steps: usize = 0;
    let mut positions: Vec<&str> = res
        .iter()
        .map(|x| *x.0)
        .filter(|x| x.chars().last().unwrap() == 'A')
        .collect();

    println!("{:?}", positions);

    let inst = inst
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
        .into_par_iter()
        .cycle();

    for i in inst {
        let over = &positions.clone().iter().fold(true, |acc, e| {
            if e.chars().nth(2).unwrap() != 'Z' {
                return false;
            }
            acc
        });

        if *over {
            break;
        }

        let mut tmp: Vec<&str> = vec![];
        for pos in &positions {
            let tup = res.get(pos).unwrap();
            //println!("{steps} {pos} {i} {tup:?}");

            let ne = match i {
                'L' => tup.0,
                'R' => tup.1,
                _ => {
                    panic!("impossible!")
                }
            };
            //println!("{steps} {pos} {i} {ne:?}");
            tmp.push(ne);
        }

        steps += 1;
        positions = tmp;
    }

    println!("{:?}", steps);

    ()
}
