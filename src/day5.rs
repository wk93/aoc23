use std::fs;

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

fn get_categories(input: &Vec<String>) -> Vec<Vec<(i64, i64, i64)>> {
    let mut categories: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut category: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.into_iter().filter(|l| !l.is_empty()) {
        match line.contains("map") {
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
        }
    }

    if !category.is_empty() {
        categories.push(category.clone());
    }

    categories
}

fn get_seeds_num(line: &str) -> Vec<i64> {
    line.split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .into_iter()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect()
}

pub fn puzzle1() {
    let mut input = get_input(false);

    let seeds = get_seeds_num(&input[0]);
    input.remove(0);
    input.remove(1);

    let categories = get_categories(&input);
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

pub fn puzzle2() {
    let mut input = get_input(false);

    let seed_nums = get_seeds_num(&input[0]);
    input.remove(0);
    input.remove(1);

    let categories_nums = get_categories(&input);

    let mut i = 0;

    let mut seeds: Vec<(i64, i64)> = Vec::new();
    while i < seed_nums.len() {
        seeds.push((seed_nums[i], seed_nums[i] - 1 + seed_nums[i + 1]));
        i += 2;
    }
    seeds.sort_unstable_by_key(|v| (v.0, v.1));

    for c in categories_nums {
        let mut categories: Vec<((i64, i64), i64)> = c
            .clone()
            .into_iter()
            .map(|c| ((c.0, c.0 + c.2 - 1), c.1 - c.0))
            .collect();

        categories.sort_unstable_by_key(|v| (v.0 .0, v.0 .1));

        let mut updated_seeds: Vec<(i64, i64)> = Vec::new();
        for seed in seeds {
            let matched_categories: Vec<((i64, i64), i64)> = categories
                .clone()
                .into_iter()
                .filter(|v| i64::max(v.0 .0, seed.0) <= i64::min(v.0 .1, seed.1))
                .collect();

            if matched_categories.len() > 0 {
                for category in matched_categories {
                    if seed.0 < category.0 .0 {
                        updated_seeds.push((seed.0, category.0 .0 - 1));
                    }

                    let overlap_0 = i64::max(seed.0, category.0 .0);
                    let overlap_1 = i64::min(seed.1, category.0 .1);
                    updated_seeds.push((overlap_0 + category.1, overlap_1 + category.1));

                    if seed.1 > category.0 .1 {
                        updated_seeds.push((category.0 .1 + 1, seed.1));
                    }
                }
            } else {
                updated_seeds.push(seed);
            }
        }
        seeds = updated_seeds;
    }

    println!(
        "{:?}",
        seeds.iter().map(|i| i.0).filter(|x| x != &0).min().unwrap()
    );
}
