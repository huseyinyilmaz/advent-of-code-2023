fn get_lines(input: &str) -> Vec<&str> {
    input
        .split('\n')
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .collect()
}

fn parse_line(line: &str) -> i32 {
    let chars: Vec<char> = line.chars().filter(|c|c.is_digit(10)).collect();
    let number_str = format!("{}{}", chars.first().unwrap(), chars.last().unwrap());
    // println!("Chars: {:?}", chars);
    // println!("number_str: {:?}", number_str);
    number_str.parse().unwrap()
}

fn calculate(input: &str) -> i32 {
    get_lines(input).into_iter().map(parse_line).sum()
}

pub fn run() {
    let input_str = include_str!("../input.txt");
    let result = calculate(input_str);
    println!("Result for day01a: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = calculate(sample_input);
        assert_eq!(result, 142);
    }
}
