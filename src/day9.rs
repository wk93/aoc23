pub fn puzzle1() {
    let lines = parse_input(get_input(false));
    let mut next_sum = 0;

    for line in lines {
        let mut v: Vec<i32> = line.clone();
        let mut next: Vec<i32> = vec![v.last().unwrap().clone()];

        while v.iter().any(|x| x != &0) {
            let mut vec_new: Vec<i32> = Vec::new();
            let mut i = 0;
            while i < v.len() - 1 {
                vec_new.push(v[i + 1] - v[i]);
                i += 1;
            }
            v = vec_new;
            next.push(v.last().unwrap().clone());
        }
        next_sum += next.iter().sum::<i32>()
    }

    println!("{}", next_sum)
}

fn parse_input(input: Vec<String>) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|x| {
            x.split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
}
fn get_input(example: bool) -> Vec<String> {
    if example {
        vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ]
    } else {
        std::fs::read_to_string("static/day9-input.txt")
            .unwrap()
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
}
