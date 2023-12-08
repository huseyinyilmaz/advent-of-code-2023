use std::{collections::HashMap};

#[derive(Debug)]
struct Input {
    route: String,
    map: HashMap<String, Vec<String>>,
}

impl From<&str> for Input {
    fn from(input_str: &str) -> Self {
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

fn calculate_steps(input: &Input, start: String) -> u128 {

    let mut move_count = 0;
    let mut current_move = start;
    loop {
        for direction in &input.route.chars().collect::<Vec<char>>() {
            if *direction == 'L' {
                current_move = input.map[&current_move][0].clone();
            } else {
                current_move = input.map[&current_move][1].clone();
            }
            move_count += 1;
            if current_move.ends_with('Z') {
                return move_count;
            }
        }
    }

}

fn lcm(a: u128, b: u128) -> u128 {
    let mut a_remaining = a;
    let mut b_remaining = b;
    let mut result: u128 = 1;
    let mut divisor = 2;
    while a_remaining > 1 || b_remaining > 1 {
        let a_rem = a_remaining % divisor;
        let b_rem = b_remaining % divisor;
        if  a_rem == 0 || b_rem == 0 {
            if a_rem == 0 {
                a_remaining = a_remaining / divisor;
            }
            if b_rem == 0 {
                b_remaining = b_remaining / divisor;
            }
            result *= divisor as u128;
        } else {
            divisor += 1
        }


    }
    result 
}

fn calculate(input: &Input) -> u128 {
    let current_moves: Vec<String> = input.map.keys().filter(|k|k.ends_with('A')).map(|k|k.clone()).collect();
    let steps: Vec<u128> = current_moves.into_iter().map(|m|calculate_steps(&input, m)).collect();
    steps.into_iter().fold(1, |acc, v| lcm(acc, v)) 
}

pub fn run() {
    let input_str = include_str!("../input.txt");

    let input = Input::from(input_str);
    let result = calculate(&input);
    println!("Result for day08b: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let result = calculate(&Input::from(sample_input));
        assert_eq!(result, 6);
    }
}
