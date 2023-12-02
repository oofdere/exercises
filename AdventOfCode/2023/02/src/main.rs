use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    #[derive(Debug)]
    struct Pull(i32, i32, i32);

    let res = contents.lines();

    // discard game ids
    let res = res
        .inspect(|x| println!("0: {x:?}"))
        .map(|l| l.split(": ").collect::<Vec<&str>>().last().unwrap().clone())
        // split at semicolons
        .map(|l| l.split("; ").collect::<Vec<&str>>())
        // split at commas, push to [red, green, blue]
        .map(|l| {
            l.iter()
                .map(|i| {
                    let mut pull: (i32, i32, i32) = (0, 0, 0);

                    let i: Vec<&str> = i.split(", ").collect();
                    for s in i {
                        let s: Vec<&str> = s.split(" ").collect();

                        match s[1] {
                            "red" => pull.0 += s[0].parse::<i32>().unwrap(),
                            "green" => pull.1 += s[0].parse::<i32>().unwrap(),
                            "blue" => pull.2 += s[0].parse::<i32>().unwrap(),
                            x => panic!("Failed to match! {x}"),
                        }
                    }

                    pull
                })
                .reduce(|acc, e| {
                    let mut e = e.clone();
                    dbg!(acc, e);
                    if acc.0 > e.0 {
                        e.0 = acc.0
                    };
                    if acc.1 > e.1 {
                        e.1 = acc.1
                    };
                    if acc.2 > e.2 {
                        e.2 = acc.2
                    };
                    e
                })
                .unwrap()
        })
        .map(|x| x.0 * x.1 * x.2)
        .inspect(|x| println!("1: {x:?}"));

    let res: i32 = res.sum();

    println!("{:?}", res);

    ()
}
