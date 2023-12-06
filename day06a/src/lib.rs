#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

#[derive(Debug)]
struct Input {
    races: Vec<Race>,
}

fn parse_input(input_str: &str) -> Input {
    let lines: Vec<&str> = input_str.lines().collect();
    let times = lines[0][5..].split_whitespace().map(|s|s.parse().unwrap());
    let distances = lines[1][9..].split_whitespace().map(|s|s.parse().unwrap());
    Input{races: times.zip(distances).map(|(time, distance)|Race{time,distance}).collect()}
}

fn calculate_race(race: Race) -> u32 {
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

fn calculate(input: Input) -> u32 {
    input.races.into_iter().map(calculate_race).product()
}
pub fn run() {
    let input_str = include_str!("../input.txt");
    let input = parse_input(input_str);
    dbg!(&input);
    let result = calculate(input);
    println!("Result for day06a: {}", result);
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
        assert_eq!(result,  288);
    }
}
