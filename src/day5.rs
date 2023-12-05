use std::{collections::HashMap, fs};

fn get_input(example: bool) -> Vec<String> {
    let filename = match example {
        true => "static/day5-input-example.txt",
        false => "static/day5-input.txt",
    };

    let mut v = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        v.push(line.to_string());
    }

    v
}

pub fn puzzle1() {
    let input = get_input(false);

    let mut seeds: Vec<i64> = Vec::new();

    let mut categories: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut category: Vec<(i64, i64, i64)> = Vec::new();

    for (i, line) in input.into_iter().filter(|l| !l.is_empty()).enumerate() {
        match i {
            0 => {
                seeds = line.split(": ").collect::<Vec<&str>>()[1]
                    .split(" ")
                    .into_iter()
                    .map(|seed| seed.parse::<i64>().unwrap())
                    .collect();
            }
            1 => {}
            _ => match line.contains("map") {
                true => {
                    if !category.is_empty() {
                        categories.push(category.clone())
                    }
                    category = Vec::new();
                }
                false => {
                    let values: Vec<i64> = line
                        .split(" ")
                        .into_iter()
                        .map(|seed| seed.parse::<i64>().unwrap())
                        .collect();

                    let key = values[1];
                    let value = values[0];
                    let range = values[2];

                    category.push((key, value, range))
                }
            },
        }
    }

    if !category.is_empty() {
        categories.push(category.clone());
    }

    let mut min = i64::MAX;

    for seed in seeds {
        let mut last = seed.to_owned();
        for map in categories.iter() {
            let mut local_map = last.to_owned();
            for value in map {
                if local_map >= value.0 && local_map <= value.2 + value.0 {
                    local_map = (value.1 + local_map - value.0).to_owned();
                    break;
                }
            }

            last = local_map;
        }

        min = i64::min(min, last)
    }

    println!("{}", min);
}
