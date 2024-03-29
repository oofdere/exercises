use std::{cmp::min, env, fs, str::FromStr};

#[derive(Debug)]
struct Present(i32, i32, i32);

impl Present {
    fn area(&self) -> i32 {
        let a = self.0 * self.1;
        let b = self.1 * self.2;
        let c = self.2 * self.0;

        let slack = min(min(a, b), c);

        slack + 2 * (a + b + c)
    }

    fn ribbon(&self) -> i32 {
        let a = self.0 + self.1;
        let b = self.1 + self.2;
        let c = self.2 + self.0;

        let min = min(min(a, b), c) * 2;
        let vol = self.0 * self.1 * self.2;

        min + vol
    }
}

impl FromStr for Present {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<i32> = s.splitn(3, 'x').map(|x| x.parse().unwrap()).collect();

        Ok(Present(s[0], s[1], s[2]))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let paper: (i32, i32) = contents
        .lines()
        .map(|x| Present::from_str(x).unwrap())
        .inspect(|x| println!("{:?}", x))
        .map(|x| (x.area(), x.ribbon()))
        .inspect(|x| println!("{:?}", x))
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
        .unwrap();

    println!("The elves need {} square feet of paper!", paper.0);
    println!("The elves need {} feet of ribbon!", paper.1);
}
