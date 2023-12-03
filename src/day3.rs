use std::fmt;
use std::fs;

struct Point {
    start: usize,
    end: usize,
    is_sign: bool,
    value: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_sign {
            write!(f, "({}): {}", self.start, "*")
        } else {
            write!(f, "({}, {}): {}", self.start, self.end, self.value)
        }
    }
}

fn get_parts(input: &Vec<String>, i: usize) -> Vec<String> {
    let input = input.iter();
    Vec::from([
        input.clone().nth(i).unwrap().to_owned(),
        match i {
            0 => "".to_string(),
            i => match input.clone().nth(i - 1) {
                Some(v) => v.to_owned(),
                None => "".to_string(),
            },
        },
        match input.clone().nth(i + 1) {
            Some(v) => v.to_owned(),
            None => "".to_string(),
        },
    ])
}

pub fn get_numbers(part: &String) -> Vec<(usize, usize, i32)> {
    let mut digits: Vec<(usize, usize, i32)> = Vec::new();

    let mut digits_start: i32 = -1;
    let mut digit_chars = "".to_string();
    for (sign_index, c) in part.chars().enumerate() {
        if c.is_digit(10) {
            if digits_start == -1 {
                digits_start = sign_index as i32;
            }
            digit_chars.push(c);
        } else {
            if digit_chars.len() > 0 {
                digits.push((
                    digits_start as usize,
                    digits_start as usize + digit_chars.len() - 1,
                    digit_chars.parse::<i32>().unwrap(),
                ));
                digits_start = -1;
                digit_chars = "".to_string();
            }
        }
    }

    if digit_chars.len() > 0 {
        digits.push((
            digits_start as usize,
            digits_start as usize + digit_chars.len() - 1,
            digit_chars.parse::<i32>().unwrap(),
        ));
    }

    digits
}

pub fn puzzle1() {
    let input = get_input(false);
    let mut sum = 0;

    for (i, _) in input.clone().iter().enumerate() {
        let mut signs: Vec<usize> = Vec::new();
        let sign_parts = get_parts(&input, i);
        let digits: Vec<(usize, usize, i32)> = get_numbers(&sign_parts[0]);

        for part in sign_parts.iter() {
            for (sign_index, c) in part.chars().enumerate() {
                if !c.is_digit(10) && c != '.' {
                    if sign_index > 0 {
                        signs.push(sign_index - 1);
                    }
                    signs.push(sign_index);
                    signs.push(sign_index + 1);
                }
            }
        }

        let i: i32 = digits
            .clone()
            .iter()
            .filter(|digit| signs.iter().any(|i| i >= &digit.0 && i <= &digit.1))
            .map(|digit| digit.2)
            .sum();
        sum += i;
    }
    println!("{}", sum);
}

pub fn puzzle() {
    let input = get_input(false);
    let mut sum = 0;

    for (i, _) in input.clone().iter().enumerate() {
        let mut signs: Vec<usize> = Vec::new();
        let mut digits: Vec<(usize, usize, i32)> = Vec::new();
        let sign_parts = get_parts(&input, i);

        for part in sign_parts.iter() {
            digits.append(&mut get_numbers(part));
        }
        for (sign_index, c) in sign_parts[0].chars().enumerate() {
            if !c.is_digit(10) && c == '*' {
                signs.push(sign_index);
            }
        }

        for sign in signs.clone().into_iter() {
            let x: Vec<i32> = digits
                .iter()
                .filter(|digit| {
                    let x = match digit.0 {
                        0 => digit.0,
                        _ => digit.0 - 1,
                    };

                    sign >= x && sign <= digit.1 + 1
                })
                .map(|digit| digit.2)
                .collect();
            if x.len() == 2 {
                sum += x[0] * x[1];
            }
        }
    }
    println!("{}", sum);
}

fn get_input(example: bool) -> Vec<String> {
    match example {
        true => vec![
            "23.4".to_string(),
            "..*.".to_string(),
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ],
        false => {
            let filename = "static/day3-input.txt";

            let mut v = Vec::new();

            for line in fs::read_to_string(filename).unwrap().lines() {
                v.push(line.to_string());
            }

            v
        }
    }
}
