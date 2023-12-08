use std::collections::HashMap;

pub fn goto(from: &str, to: char, map: &HashMap<String, (String, String)>) -> String {
    let point = map.get(from).unwrap();

    if to == 'L' {
        point.0.to_string()
    } else {
        point.1.to_string()
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut max = a;
    let mut min = b;

    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res
    }
}

pub fn puzzle2() {
    let (steps, map) = parse_input(get_input(false));
    let steps = steps.chars().collect::<Vec<char>>();
    let mut selected: Vec<String> = map
        .keys()
        .filter(|k| k.chars().next_back().unwrap() == 'A')
        .map(|k| k.to_string())
        .collect();

    let mut count: Vec<i32> = Vec::new();
    let mut step = 0;

    let mut i = 0;
    let mut c = 1;
    while count.len() != selected.len() {
        selected[i] = goto(&selected[i], steps[step], &map);

        if selected[i].chars().next_back().unwrap() == ('Z') {
            count.push(c);
            c = 1;
            step = 0;
            i += 1;
        } else {
            c += 1;
            step += 1;

            if step == steps.len() {
                step = 0;
            }
        }
    }

    let mut result = 1;

    for v in count {
        result = lcm(result, v as u64);
    }

    println!("{}", result);
}

pub fn puzzle1() {
    let (steps, map) = parse_input(get_input(false));
    let mut selected: String = "AAA".to_string();
    let mut count = 0;
    let mut step = 0;

    let steps = steps.chars().collect::<Vec<char>>();

    while selected != "ZZZ" {
        selected = goto(&selected, steps[step], &map);
        step += 1;

        if step == steps.len() {
            step = 0;
        }
        count += 1;
    }

    println!("{}", count);
}

fn parse_input(input: Vec<String>) -> (String, HashMap<String, (String, String)>) {
    let steps = &input[0];
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in input.iter().skip(2) {
        let splited_line = line.split(" = ").collect::<Vec<&str>>();
        let index = splited_line[0];

        let mut chars = splited_line[1].chars();
        chars.next();
        chars.next_back();

        let nodes = chars.as_str().split(", ").collect::<Vec<&str>>();

        map.insert(
            index.to_string(),
            (nodes[0].to_string(), nodes[1].to_string()),
        );
    }

    (steps.to_string(), map)
}

fn get_input(example: bool) -> Vec<String> {
    if example {
        vec![
            "LR".to_string(),
            "".to_string(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ]
    } else {
        let filename = "static/day8-input.txt";

        std::fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
}
