use std::fs;

fn get_input(example: bool) -> Vec<String> {
    match example {
        true => vec![
            "two1ninenine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ],
        false => {
            let filename = "static/day1-input.txt";

            let mut v2 = Vec::new();

            for line in fs::read_to_string(filename).unwrap().lines() {
                v2.push(line.to_string());
            }

            v2
        }
    }
}

fn calibrate(line: &String) -> i32 {
    let mut numbers = line.chars().filter(|c| c.is_numeric());

    match numbers.clone().count() {
        0 => 0,
        1 => {
            let num = numbers.nth(0).unwrap();
            let mut str = num.to_string();
            str.push(num);

            str.parse::<i32>().unwrap()
        }
        _ => {
            let mut str = numbers.nth(0).unwrap().to_string();
            str.push(numbers.last().unwrap());
            str.parse::<i32>().unwrap()
        }
    }
}

fn transform_digits(line: &String) -> String {
    let spelled = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut first_digit = (i32::MAX, "".to_string());
    let mut last_digit = (-1, "".to_string());
    let mut updated_line = line.clone();


    for spell in spelled.iter() {
        let x = line.find(spell.0);
        let y = line
            .chars()
            .rev()
            .collect::<String>()
            .find(&spell.0.chars().rev().collect::<String>());
        match x {
            Some(x) => {
                if x < first_digit.0.try_into().unwrap() {
                    first_digit = (x as i32, spell.1.to_string());
                }
            }
            None => {}
        };
        match y {
            Some(y) => {
                let line_length = line.len() as i32;
                let spell_length = spell.0.len() as i32;
                let position: i32 = line_length - spell_length - y as i32 + 1;

                if position > last_digit.0.try_into().unwrap() {
                    last_digit = (position as i32, spell.1.to_string());
                }
            }
            None => {}
        };
    }

    if first_digit.0 != i32::MAX {
        updated_line.insert_str(first_digit.0 as usize, &first_digit.1);

        if first_digit.0 != last_digit.0 {
            updated_line.insert_str(last_digit.0 as usize, &last_digit.1);
        }
    }

    updated_line
}

pub fn puzzle1() {
    let sum: i32 = get_input(false).iter().map(calibrate).sum();
    println!("{}", sum);
}

pub fn puzzle2() {
    let sum: i32 = get_input(false)
        .iter()
        .map(|line| transform_digits(line))
        .map(|line| calibrate(&line))
        .sum();

    println!("{}", sum);
}
