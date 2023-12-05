use std::{cell::UnsafeCell, collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("failed to read file");

    let lines = contents.lines().collect::<Vec<&str>>();

    let seeds = lines
        .iter()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("seeds: {seeds:?}");

    let to_soil = to_map("to-soil", &lines);

    let to_fertilizer = to_map("to-fertilizer", &lines);

    let to_water = to_map("to-water", &lines);

    let to_light = to_map("to-light", &lines);

    let to_temp = to_map("to-temperature", &lines);

    let to_humid = to_map("to-humidity", &lines);

    let to_location = to_map("to-location", &lines);

    // generate location numbers for each seed
    let locations = seeds
        .iter()
        .inspect(|x| println!("{x:?}"))
        .map(|seed| {
            let mut settings: Vec<usize> = vec![*seed];

            let soil: &usize = to_soil.get(seed).unwrap_or(seed);
            settings.push(*soil);

            let fert: &usize = to_fertilizer.get(soil).unwrap_or(soil);
            settings.push(*fert);

            let water: &usize = to_water.get(fert).unwrap_or(fert);
            settings.push(*water);

            let light: &usize = to_light.get(water).unwrap_or(water);
            settings.push(*light);

            let temp: &usize = to_temp.get(light).unwrap_or(light);
            settings.push(*temp);

            let humid: &usize = to_humid.get(temp).unwrap_or(temp);
            settings.push(*humid);

            let loc: &usize = to_location.get(humid).unwrap_or(humid);
            settings.push(*loc);

            println!("{:?}", &settings);

            loc
        })
        .collect::<Vec<&usize>>();

    println!("{:?}", locations)
}

fn to_map(heading: &str, content: &Vec<&str>) -> HashMap<usize, usize> {
    // scroll to header
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
