// use std::collections::HashMap;

use std::ops::Not;

type ReturnType = u128;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Stone,
    Ash,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Stone),
            '.' => Ok(Tile::Ash),
            _ => Err("Cannot Parse Tile"),
        }
    }
}

impl Not for Tile {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Ash => Self::Stone,
            Self::Stone => Self::Ash,
        }
    }
}

type Pattern = Vec<Vec<Tile>>;

#[derive(Debug)]
struct Input {
    patterns: Vec<Pattern>,
}

fn format_pattern(pattern: &Pattern) -> String {
    let mut result = String::new();
    for line in pattern {
        for tile in line {
            result += match tile {
                Tile::Ash => ".",
                Tile::Stone => "#",
            };
        }
        result += "\n";
    }
    result += "\n";
    result
}

impl ToString for Input {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for pattern in self.patterns.iter() {
            result = format!("{}{}\n", result, format_pattern(pattern));
        }
        result
    }
}

impl TryFrom<&str> for Input {
    type Error = &'static str;

    fn try_from(input_str: &str) -> Result<Self, Self::Error> {
        let mut patterns = Vec::new();
        let mut pattern = Vec::new();
        for line in input_str.lines() {
            if line.is_empty() {
                if !pattern.is_empty() {
                    patterns.push(pattern);
                    pattern = Vec::new();
                }
            } else {
                // Line is not empty
                let pattern_line = line
                    .chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<Tile>, Self::Error>>()?;
                pattern.push(pattern_line);
            }
        }
        if !pattern.is_empty() {
            patterns.push(pattern);
        }
        Ok(Input { patterns })
    }
}

fn is_row_equal(pattern: &Pattern, lower: i32, upper: i32) -> bool {
    for x in 0..pattern[0].len() {
        if pattern[lower as usize][x] != pattern[upper as usize][x] {
            return false;
        }
    }
    true
}

fn is_row_mirrow(pattern: &Pattern, lower: i32, upper: i32) -> bool {
    let mut lower_limit = lower;
    let mut upper_limit = upper;
    while lower_limit >= 0 && upper_limit < pattern.len() as i32 {
        if is_row_equal(&pattern, lower_limit, upper_limit) {
            lower_limit -= 1;
            upper_limit += 1;
        } else {
            return false;
        }
    }
    true
}

fn is_col_equal(pattern: &Pattern, lower: i32, upper: i32) -> bool {
    for y in 0..pattern.len() {
        if pattern[y][lower as usize] != pattern[y][upper as usize] {
            return false;
        }
    }
    true
}

fn is_col_mirrow(pattern: &Pattern, lower: i32, upper: i32) -> bool {
    let mut lower_limit = lower;
    let mut upper_limit = upper;
    while lower_limit >= 0 && upper_limit < pattern[0].len() as i32 {
        if is_col_equal(&pattern, lower_limit, upper_limit) {
            lower_limit -= 1;
            upper_limit += 1;
        } else {
            return false;
        }
    }
    true
}

fn get_pattern_score(pattern: &Pattern, ignore: Option<ReturnType>) -> Option<ReturnType> {
    for x in 0..pattern[0].len() - 1 {
        if is_col_mirrow(&pattern, x as i32, (x + 1) as i32) {
            println!("column mirror found! {},{}", x, x + 1);
            println!("{}", format_pattern(&pattern));
            let result = ((x + 1) * 1) as ReturnType;
            match ignore {
                None => return Some(result),
                Some(other_result) => {
                    if other_result != result {
                        return Some(result);
                    }
                }
            }
        }
    }
    for y in 0..(pattern.len() - 1) {
        if is_row_mirrow(&pattern, y as i32, (y + 1) as i32) {
            println!("column mirror found! {},{}", y, y + 1);
            println!("{}", format_pattern(&pattern));
            let result = ((y + 1) * 100) as ReturnType;
            match ignore {
                None => return Some(result),
                Some(other_result) => {
                    if other_result != result {
                        return Some(result);
                    }
                }
            }
        }
    }
    None
}

fn calculate(input: &mut Input) -> ReturnType {
    // println!("{}", input.to_string());
    let mut result: ReturnType = 0;
    'pattern_loop: for pattern in &input.patterns {
        println!("Original Pattern Score");
        let original_score: ReturnType =
            get_pattern_score(&pattern, None).expect("Could not find Mirror");

        println!("Test Pattern Scores");
        let mut test_pattern: Pattern = pattern.clone();
        for y in 0..pattern.len() {
            for x in 0..pattern[y].len() {
                // flip a single tile to test
                test_pattern[y][x] = !test_pattern[y][x];
                if let Some(new_score) = get_pattern_score(&test_pattern, Some(original_score)) {
                    if new_score != original_score {
                        result += new_score;
                        continue 'pattern_loop;
                    }
                }
                // return test_pattern to its original state
                test_pattern[y][x] = !test_pattern[y][x];
            }
        }
    }
    result
}

pub fn run() {
    let input_str = include_str!("../input.txt");
    let mut input = Input::try_from(input_str).unwrap();
    let result = calculate(&mut input);
    println!("Result for day11a: {}", result);
}
// 42361
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 400);
    }
}






