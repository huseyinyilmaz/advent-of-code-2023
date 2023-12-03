
fn get_lines(input: &str) -> Vec<&str> {
    input
        .split('\n')
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .collect()
}

fn parse_line(line: &str) -> i32 {
    let number_str = format!("{}{}", get_first_digit(line).unwrap(), get_last_digit(line).unwrap());
    number_str.parse().unwrap()
}

fn get_first_digit(line: &str) -> Option<&str> {
    let nums = vec![
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
    let mut rest = line;
    while !rest.is_empty() {
        if rest.chars().next().unwrap().is_digit(10) {
            return Some(&rest[0..1]);
        }
        for (spelled, num) in &nums {
            if rest.starts_with(spelled) {
                return Some(num);
            }
        }
        rest = &rest[1..];
    }
    return None;
}

fn get_last_digit(line: &str) -> Option<&str> {
    let nums = vec![
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
    let mut rest = line;
    while !rest.is_empty() {
        if rest.chars().last().unwrap().is_digit(10) {
            return Some(&rest[rest.len()-1..]);
        }
        for (spelled, num) in &nums {
            if rest.ends_with(spelled) {
                return Some(num);
            }
        }
        rest = &rest[..rest.len()-1];
    }
    return None;
}

fn calculate(input: &str) -> i32 {
    get_lines(input)
        .into_iter()
        .map(parse_line)
        .sum()
}

pub fn run() {
    let input_str = include_str!("../input.txt");
    let result = calculate(input_str);
    println!("Result for day01b: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = calculate(sample_input);
        assert_eq!(result, 281);
    }
}
