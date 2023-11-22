use std::{collections::HashMap, env, fs};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point(i32, i32);

impl Point {
    fn make_move(&mut self, c: char) -> Point {
        *self = match c {
            '^' => Point(self.0, self.1 + 1),
            'v' => Point(self.0, self.1 - 1),
            '>' => Point(self.0 + 1, self.1),
            '<' => Point(self.0 - 1, self.1),
            _ => panic!(),
        };

        *self
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let mut map: HashMap<Point, i32> = HashMap::new();
    let mut pos = Point(0, 0);

    map.insert(pos, 1);
    contents
        .chars()
        .map(|x| pos.make_move(x))
        .inspect(|x| println!("{:?}", x))
        .for_each(|x| {
            map.entry(x).and_modify(|x| *x += 1).or_insert(1);
        });

    let houses = map.iter().count();

    println!("{} houses receive at least one present!", houses)
}
