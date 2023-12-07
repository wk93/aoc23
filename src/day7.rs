use std::fs;

pub fn puzzle1() {
    let mut sum = 0;
    let mut hands = parse_input(get_input(false));

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    for (i, hand) in hands.iter().enumerate() {
        sum += hand.1 * (i as i32 + 1);
    }

    println!("{:?}", sum);
}

pub fn puzzle2() {
    let mut sum = 0;
    let mut hands = parse_input_puzzle2(get_input(false));

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    for (i, hand) in hands.iter().enumerate() {
        if hand.0 .0 == 0 || hand.0 .1 == 0 || hand.0 .2 == 0 || hand.0 .3 == 0 || hand.0 .4 == 0 {
            println!("{:?}", hand);
        }
        sum += hand.1 * (i as i32 + 1);
    }

    println!("{:?}", sum);
}

fn parse_input_puzzle2(input: Vec<String>) -> Vec<((i32, i32, i32, i32, i32, i32), i32)> {
    let mut hands: Vec<((i32, i32, i32, i32, i32, i32), i32)> = Vec::new();
    for line in input {
        let splited_line = line.split(" ").collect::<Vec<&str>>();
        let cards = splited_line[0];
        let bid = splited_line[1].parse().unwrap();
        let mut card_values: Vec<i32> = Vec::new();
        let mut repeat_values: Vec<(i32, i32)> = Vec::new();
        let mut jokers_count = 0;

        for card in cards.chars() {
            let num: i32 = match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    jokers_count += 1;
                    0
                }
                'T' => 10,
                _ => card.to_digit(10).unwrap() as i32,
            };
            card_values.push(num);

            match repeat_values
                .clone()
                .into_iter()
                .map(|x| x.1)
                .position(|x| x == num)
            {
                Some(i) => {
                    repeat_values[i].0 = repeat_values[i].0 + 1;
                }
                None => {
                    if num > 0 {
                        repeat_values.push((1, num.try_into().unwrap()))
                    }
                }
            };
        }

        repeat_values.sort_by(|a, b| b.cmp(a));
        let mut hand_value = 1;

        if repeat_values.len() == 0 {
            hand_value = 7
        } else if repeat_values[0].0 == 5 {
            hand_value = 7;
        } else if repeat_values[0].0 == 4 {
            hand_value = 6;
        } else if repeat_values[0].0 == 3 && repeat_values.len() > 1 && repeat_values[1].0 == 2 {
            hand_value = 5;
        } else if repeat_values[0].0 == 3 {
            hand_value = 4;
        } else if repeat_values[0].0 == 2 && repeat_values.len() > 1 && repeat_values[1].0 == 2 {
            hand_value = 3;
        } else if repeat_values[0].0 == 2 {
            hand_value = 2;
        }

        for j in 0..jokers_count {
            if hand_value == 2 || hand_value == 4 || hand_value == 3 {
                hand_value += 2;
            } else {
                hand_value = i32::min(hand_value + j + 1, 7);
            }
        }

        hands.push((
            (
                hand_value,
                card_values[0],
                card_values[1],
                card_values[2],
                card_values[3],
                card_values[4],
            ),
            bid,
        ));
    }

    hands
}

fn parse_input(input: Vec<String>) -> Vec<((i32, i32, i32, i32, i32, i32), i32)> {
    let mut hands: Vec<((i32, i32, i32, i32, i32, i32), i32)> = Vec::new();
    for line in input {
        let splited_line = line.split(" ").collect::<Vec<&str>>();
        let cards = splited_line[0];
        let bid = splited_line[1].parse().unwrap();
        let mut card_values: Vec<i32> = Vec::new();
        let mut repeat_values: Vec<(i32, i32)> = Vec::new();

        for card in cards.chars() {
            let num: i32 = match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => card.to_digit(10).unwrap() as i32,
            };

            card_values.push(num);

            match repeat_values
                .clone()
                .into_iter()
                .map(|x| x.1)
                .position(|x| x == num)
            {
                Some(i) => {
                    repeat_values[i].0 = repeat_values[i].0 + 1;
                }
                None => repeat_values.push((1, num.try_into().unwrap())),
            };
        }

        repeat_values.sort_by(|a, b| b.cmp(a));
        let mut hand_value = 1;

        if repeat_values.len() == 1 {
            hand_value = 7;
        } else if repeat_values.len() == 2 && repeat_values[0].0 == 4 {
            hand_value = 6;
        } else if repeat_values.len() == 2 {
            hand_value = 5;
        } else if repeat_values.len() == 3 && repeat_values[0].0 == 3 {
            hand_value = 4;
        } else if repeat_values.len() == 3 {
            hand_value = 3;
        } else if repeat_values.len() == 4 {
            hand_value = 2;
        }

        hands.push((
            (
                hand_value,
                card_values[0],
                card_values[1],
                card_values[2],
                card_values[3],
                card_values[4],
            ),
            bid,
        ));
    }

    hands
}

fn get_input(example: bool) -> Vec<String> {
    match example {
        true => vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ],
        false => {
            let filename = "static/day7-input.txt";

            let mut v = Vec::new();

            for line in fs::read_to_string(filename).unwrap().lines() {
                v.push(line.to_string());
            }

            v
        }
    }
}
