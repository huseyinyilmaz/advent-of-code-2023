type Reading = Vec<i32>;

#[derive(Debug)]
struct Input {
    readings: Vec<Reading>,
}

impl From<&str> for Input {
    fn from(input_str: &str) -> Self {
        let readings = input_str
            .lines()
            .filter(|l|!l.is_empty())
            .map(|l|l.split_whitespace().map(|n|n.parse().unwrap()).collect())
            .collect();
        Input { readings }
    }
}

fn get_next_number(reading: Reading) -> i32 {
    if reading.iter().all(|n|*n == 0) {
        0
    } else {
        let sub_reading: Reading = reading.iter().zip(&reading[1..]).map(|(first,second)| second - first).collect();
        reading.last().unwrap() + get_next_number(sub_reading)
    }
}

fn calculate(input: Input) -> i32 {
    input.readings.into_iter().map(get_next_number).sum()
}

pub fn run() {
    let input_str = include_str!("../input.txt");

    let input = Input::from(input_str);
    let result = calculate(input);
    println!("Result for day09a: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let result = calculate(Input::from(sample_input));
        assert_eq!(result, 114);
    }
}
