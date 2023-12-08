use std::{cell::UnsafeCell, cmp::Ordering, collections::HashMap, env, fs};

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
        .collect::<HashMap<&str, (&str, &str)>>();

    println!("{:?}", res);

    let mut steps = 0;
    let mut pos = "AAA";

    let inst = inst.chars().cycle();

    for i in inst {
        let tup = res.get(pos).unwrap();
        println!("{steps} {pos} {tup:?}");

        let ne = match i {
            'L' => tup.0,
            'R' => tup.1,
            _ => {
                panic!("impossible!")
            }
        };
        println!("{steps} {pos} {ne:?}");

        steps += 1;
        pos = ne;

        if pos == "ZZZ" {
            break;
        }
    }

    println!("{:?}", steps);

    ()
}
