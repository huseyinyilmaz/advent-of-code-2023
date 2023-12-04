use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: i32,
    wining_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

fn get_lines(input: &str) -> Vec<&str> {
    input
        .lines()
        // .split('\n')
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .collect()
}

fn parse_card(line: &str) -> Card {
    let (game_id_raw, game_id_rest) = line.split_once(':').unwrap();
    let game_id = game_id_raw[5..].trim().parse().unwrap();
    println!("id: {}", game_id);
    let (wining_numbers_str, numbers_str) = game_id_rest.split_once('|').unwrap();
    dbg!(wining_numbers_str, numbers_str);
    Card {
        id: game_id,
        wining_numbers: wining_numbers_str
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect(),
        numbers: numbers_str
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect(),
    }
}

fn process_card(card: Card) -> i32 {
    println!("card: {:?}", card);
    let wining_numbers: HashSet<i32> = HashSet::from_iter(card.wining_numbers);
    let mut result = 0;
    for n in card.numbers {
        if wining_numbers.contains(&n) {
            if result == 0 {
                result = 1;
            } else {
                result *= 2;
            }
        }
    }
    result
}

fn calculate(input: &str) -> i32 {
    get_lines(input)
        .into_iter()
        .map(parse_card)
        .inspect(|c| {
            dbg!(c);
        })
        .map(process_card)
        .sum()
}

pub fn run() {
    let input_str = include_str!("../input.txt");
    let result = calculate(input_str);
    println!("Result for day04a: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let result = calculate(sample_input);
        assert_eq!(result, 13);
    }
}
