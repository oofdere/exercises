use std::{cell::UnsafeCell, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    #[derive(Debug)]
    struct Pull(i32, i32, i32);

    let lines = contents.lines();

    let res: Vec<(usize, Vec<(usize, char)>)> = lines
        .clone()
        .map(|x| x.chars().enumerate().collect())
        .enumerate()
        .inspect(|x| println!("{:?} {:?}", x.0, x.1))
        .collect();

    // get all indexes into res where there is a symbol
    let symbols: Vec<(usize, usize)> = res
        .iter()
        .map(|x| {
            return x
                .1
                .iter()
                .filter_map(|y| {
                    // is y not a dot or number
                    if y.1 == '.' || y.1.is_numeric() {
                        return None;
                    }
                    return Some((x.0, y.0));
                })
                .collect::<Vec<(usize, usize)>>();
        })
        .inspect(|x| println!("{:?}", x))
        .flatten()
        .collect();

    println!("locations: {:?}", symbols);

    let mut res: Vec<Vec<char>> = lines.map(|l| l.chars().collect::<Vec<char>>()).collect();

    // search around those indexes to see if there is a number, if there is, return the index of that number
    let nums: i32 = symbols
        .iter()
        .map(|(x, y)| {
            let mut n: Vec<i32> = vec![];
            println!("({x}, {y})");

            for i in x - 1..x + 2 {
                for j in y - 1..y + 2 {
                    print!("{}", res[i][j]);
                    if res[i][j].is_numeric() {
                        let num = extract_num(&mut res[i], j).parse::<i32>().unwrap();
                        // also makes number too low
                        // if !n.contains(&num) {
                        n.push(num)
                    }
                }
                println!()
            }

            // thankfully part 2 requires minimal changes
            if n.len() == 2 {
                return n[0] * n[1];
            }

            0
        })
        .sum();

    // forward and backward search to extract the number
    println!("{nums}");
    ()
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
