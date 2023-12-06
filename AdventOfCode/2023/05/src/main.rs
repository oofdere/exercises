use std::{cell::UnsafeCell, collections::HashMap, env, fs, ops::Range};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let lines = contents.lines().collect::<Vec<&str>>();

    // create a range map
    // which is a map with a range as the key
    // and an offset range as the value

    let res: HashMap<Range<usize>, (usize, usize)> = ();
}

fn to_map(
    heading: &str,
    content: &Vec<&str>,
    map: &mut HashMap<usize, usize>,
) -> HashMap<usize, usize> {
    // scroll to headerrust r
    let content = content.iter().skip_while(|x| !x.contains(heading)).skip(1);

    // map and collect
    let content = content
        .map_while(|x| {
            if x.is_empty() {
                return None;
            };

            Some(x)
        })
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .map(|x| {
            x.iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|x| {
            let mut res: Vec<(usize, usize)> = vec![];

            // whoever decided from should be the second element,
            // please remember those of us without
            // reading comprehension next time
            let from = x[1];
            let to = x[0];
            // TIMES MAY NOT EXIST
            // ADD CODE TO ADDRESS THAT
            // PROBABLY USE .GET AND THEN .UNWRAP_OR
            let times = x.get(2).unwrap_or(&0);

            for i in 0..*times {
                res.push((from + i, to + i))
            }

            res
        })
        .flatten()
        .collect::<HashMap<usize, usize>>();

    println!("{heading}: {:?}", content);

    content
}
