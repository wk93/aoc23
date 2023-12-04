use std::fs;

fn parse_str_to_numbers(line: &str) -> Vec<i32> {
    line.trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|value| !value.is_empty())
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn parse_line(line: &str) -> usize {
    let splitted_line: Vec<&str> = line.split(" | ").collect();
    let winning_numbers =
        parse_str_to_numbers(splitted_line[0].split(":").collect::<Vec<&str>>()[1]);
    let numbers = parse_str_to_numbers(splitted_line[1]);

    numbers
        .into_iter()
        .filter(|val| winning_numbers.contains(val))
        .collect::<Vec<i32>>()
        .len()
}

pub fn puzzle1() {
    let input = get_input(false);
    let mut total_points = 0;

    for line in input {
        let sum = parse_line(&line);
        let points = match sum {
            0 => 0,
            _ => i32::pow(2, sum as u32 - 1),
        };
        total_points += points;
    }

    println!("{}", total_points);
}

pub fn puzzle2() {
    let input = get_input(false);
    let len = input.len();
    let mut cards: Vec<i32> = Vec::new();

    for (i, line) in input.into_iter().enumerate() {
        let mut instances: usize = match cards.iter().nth(i) {
            Some(v) => v.to_owned() as usize,
            None => {
                cards.push(0);
                0
            }
        };
        instances += 1;
        cards[i] = instances as i32;

        let won_cards = usize::min(parse_line(&line), len - 1 - i);

        for j in i + 1..i + 1 + won_cards {
            match cards.clone().into_iter().nth(j) {
                None => {
                    cards.push(instances as i32);
                }
                Some(v) => {
                    cards[j] = v + instances as i32;
                }
            }
        }
    }

    println!("{:?}", cards.iter().sum::<i32>());
}

fn get_input(example: bool) -> Vec<String> {
    match example {
        true => vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ],
        false => {
            let filename = "static/day4-input.txt";

            let mut v = Vec::new();

            for line in fs::read_to_string(filename).unwrap().lines() {
                v.push(line.to_string());
            }

            v
        }
    }
}
