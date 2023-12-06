#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_input(input_str: &str) -> Race {
    let lines: Vec<&str> = input_str.lines().collect();
    let time: u64 = lines[0][5..].chars().filter(|s|s.is_digit(10)).collect::<String>().parse().unwrap();
    let distance: u64 = lines[1][9..].chars().filter(|s|s.is_digit(10)).collect::<String>().parse().unwrap();
    Race{ time, distance}    
}

fn calculate_race(race: Race) -> u64 {
    let mut result = 0;
    for time in 0..race.time {
        let speed = race.time - time;
        let distance = speed * time;
        if distance > race.distance {
            result += 1;
        }
    }
    dbg!(race);
    dbg!(&result);
    result
}

fn calculate(race: Race) -> u64 {
    calculate_race(race) 
}
pub fn run() {
    let input_str = include_str!("../input.txt");
    let input = parse_input(input_str);
    dbg!(&input);
    let result = calculate(input);
    println!("Result for day06b: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "Time:      7  15   30
Distance:  9  40  200
";
        let result = calculate(parse_input(sample_input));
        assert_eq!(result, 71503);
    }
}
