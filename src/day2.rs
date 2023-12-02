use std::fs;

const MAX_RED_CUBES: usize = 12;
const MAX_GREEN_CUBES: usize = 13;
const MAX_BLUE_CUBES: usize = 14;

fn get_input(example: bool) -> Vec<String> {
    match example {
        true => vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ],
        false => {
            let filename = "static/day2-input.txt";

            let mut v = Vec::new();

            for line in fs::read_to_string(filename).unwrap().lines() {
                v.push(line.to_string());
            }

            v
        }
    }
}

pub fn parse_line(line: &str) -> (usize, usize) {
    let mut splited_line = line.split(":");

    let game_id_part = splited_line.next().unwrap().split(" ");
    let mut id = game_id_part.last().unwrap().parse::<usize>().unwrap();

    let sets = splited_line.next().unwrap().split(";");

    let mut red: usize = 0;
    let mut green: usize = 0;
    let mut blue: usize = 0;
    for set in sets {
        let cubes = set.split(",");

        for cube in cubes {
            let mut splited_cube = cube.trim().split(" ");
            let count = splited_cube.next().unwrap().parse::<usize>().unwrap();
            let color = splited_cube.next().unwrap();

            match color {
                "red" => {
                    if count > MAX_RED_CUBES {
                        id = 0;
                    }
                    if count > red {
                        red = count;
                    }
                }
                "green" => {
                    if count > MAX_GREEN_CUBES {
                        id = 0;
                    }
                    if count > green {
                        green = count;
                    }
                }
                "blue" => {
                    if count > MAX_BLUE_CUBES {
                        id = 0;
                    }
                    if count > blue {
                        blue = count;
                    }
                }
                _ => {}
            }
        }
    }

    (id, red * green * blue)
}

pub fn puzzle() {
    let input = get_input(false);

    let mut sum = (0, 0);

    for line in input {
        sum.0 += parse_line(&line).0;
        sum.1 += parse_line(&line).1;
    }

    println!("puzzle1: {} puzzle2: {}", sum.0, sum.1);
}
