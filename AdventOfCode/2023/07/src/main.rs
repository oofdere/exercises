use std::{cell::UnsafeCell, cmp::Ordering, collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let lines = contents.lines();

    let mut res = lines
        .map(|x| x.split_once(" ").unwrap())
        .map(|(hand, bid)| {
            let mut h: Vec<usize> = vec![];

            for card in hand.chars() {
                h.push(match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    x => x.to_digit(10).unwrap() as usize,
                });
            }
            (h, bid.parse::<usize>().unwrap())
        })
        .map(|(hand, bid)| (HandToRank(&hand), hand, bid))
        .collect::<Vec<(usize, Vec<usize>, usize)>>();

    println!("{:?}", res);

    res.sort_by(|x, y| match x.0.cmp(&y.0) {
        Ordering::Equal => {
            for i in 0..5 {
                match x.1[i].cmp(&y.1[i]) {
                    Ordering::Equal => continue,
                    x => return x,
                }
            }
            Ordering::Equal
        }
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
    });

    println!("{:?}", res);

    let res: usize = res
        .iter()
        .map(|x| x.2)
        .enumerate()
        .inspect(|x| println!("{:?}, {:?}", x.0 + 1, x.1))
        .map(|(idx, bid)| (idx + 1) * bid)
        .sum();

    println!("{:?}", res);

    ()
}

fn HandToRank(hand: &Vec<usize>) -> usize {
    // map for freq information
    let mut map: HashMap<usize, usize> = HashMap::new();

    // fill map
    for card in hand {
        *map.entry(card.clone()).or_default() += 1;
    }

    let mut freq: Vec<usize> = map.iter().map(|(_, x)| *x).collect::<Vec<usize>>();
    freq.sort();

    match freq.last().unwrap() {
        5 => 7, // five of a kind = 7
        4 => 6, // four of a kind = 6
        3 => {
            if freq.len() == 2 {
                return 5; // full house = 5
            }
            4 // three of a kind = 4
        }
        2 => {
            if freq[2] == 2 {
                return 3; // two pair = 3
            }
            2 // one pair = 2
        }
        _ => 1, // high card = 1
    }
}
