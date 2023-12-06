use std::fs;
pub fn puzzle1() {
    let races = parse_input(get_input(false));

    let mut multiple: usize = 1;
    for race in races {
        let mut options: Vec<i32> = Vec::new();
        let duration = race.0;
        let record = &race.1;
        for x in 0..duration {
            options.push(x * (duration - x));
        }
        let sum = options.iter().filter(|distance| distance > &record).count();

        multiple *= sum;
    }

    println!("{}", multiple);
}

pub fn puzzle2() {
    let race = parse_input_puzzle2(get_input(false));
    let mut multiple: usize = 1;
    let mut options: Vec<i64> = Vec::new();
    let duration = race.0;
    let record = &race.1;
    for x in 0..duration {
        options.push(x * (duration - x));
    }
    let sum = options.iter().filter(|distance| distance > &record).count();

    multiple *= sum;

    println!("{}", multiple);
}

fn parse_input_puzzle2(input: Vec<String>) -> (i64, i64) {
    let time_line = input[0].split(":").collect::<Vec<&str>>()[1];
    let distance_line = input[1].split(":").collect::<Vec<&str>>()[1];

    let time = time_line.chars().filter(|c| c != &' ').collect::<String>();
    let distance = distance_line
        .chars()
        .filter(|c| c != &' ')
        .collect::<String>();

    (
        time.parse::<i64>().unwrap(),
        distance.parse::<i64>().unwrap(),
    )
}

fn parse_input(input: Vec<String>) -> Vec<(i32, i32)> {
    let mut races: Vec<(i32, i32)> = Vec::new();

    for line in input {
        let nums = line.split(":").collect::<Vec<&str>>()[1]
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for (i, num) in nums.iter().enumerate() {
            match races.get_mut(i) {
                Some(race) => {
                    race.1 = num.to_owned();
                }
                None => {
                    races.push((num.to_owned(), 0));
                }
            }
        }
    }

    races
}

fn get_input(example: bool) -> Vec<String> {
    match example {
        true => vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ],
        false => {
            let filename = "static/day6-input.txt";

            let mut v = Vec::new();

            for line in fs::read_to_string(filename).unwrap().lines() {
                v.push(line.to_string());
            }

            v
        }
    }
}
