use std::{collections::HashMap};

#[derive(Debug)]
struct Input {
    route: String,
    map: HashMap<String, Vec<String>>,
}

impl From<&str> for Input {
    fn from(input_str: &str) -> Self {
        println!("input_str: {}", input_str);
        let mut lines = input_str.lines();
        let route = lines.next().unwrap().to_string();
        assert!(lines.next() == Some(""));
        let mut map = HashMap::new();
        for line in lines {
            let parts: Vec<&str> = line.split(" = ").collect();
            let key = parts[0];
            let mut last = parts[1];
            // last = (BBB, CCC)
            // remove parntesis
            last = &last[1..last.len() - 1];
            // last = BBB, CCC
            let value: Vec<String> = last.split(", ").map(String::from).collect();
            assert!(value.len() == 2);
            map.insert(key.to_string(), value);
        }

        Input { route, map }
    }
}

fn calculate(input: Input) -> u32 {
    let mut move_count = 0;
    let mut current_move = "AAA".to_string();
    loop {
        for direction in &input.route.chars().collect::<Vec<char>>() {
            if *direction == 'L' {
                current_move = input.map[&current_move][0].clone();
            } else {
                current_move = input.map[&current_move][1].clone();
            }
            move_count += 1;
            if current_move == "ZZZ" {
                return move_count;
            }
        }
    }
}
pub fn run() {
    let input_str = include_str!("../input.txt");

    let input = Input::from(input_str);
    let result = calculate(input);
    println!("Result for day08a: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        let result = calculate(Input::from(sample_input));
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works2() {
        let sample_input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        let result = calculate(Input::from(sample_input));
        assert_eq!(result, 6);
    }
}
