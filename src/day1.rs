use std::fs;

pub fn puzzle1() {
    let filename = "static/day1-input.txt";

    let mut v2 = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        v2.push(line.to_string());
    }

    let sum: i32 = v2
        .iter()
        .map(|line| {
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
        })
        .sum();

    println!("{}", sum);
}
