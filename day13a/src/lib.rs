// use std::collections::HashMap;

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

type Grid = Vec<Vec<Tile>>;

#[derive(Debug)]
struct Input {
    patterns: Vec<Grid>,
}

fn format_pattern(pattern: &Grid) -> String {
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

fn is_row_equal(pattern: &Grid, lower: i32, upper: i32) -> bool {
    for x in 0..pattern[0].len() {
        if pattern[lower as usize][x] != pattern[upper as usize][x] {
            return false;
        }
    }
    true
}

fn is_row_mirrow(pattern: &Grid, lower: i32, upper: i32) -> bool {
    println!("is mirror: {} == {}", lower, upper);
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

fn is_col_equal(pattern: &Grid, lower: i32, upper: i32) -> bool {
    for y in 0..pattern.len() {
        if pattern[y][lower as usize] != pattern[y][upper as usize] {
            return false;
        }
    }
    true
}

fn is_col_mirrow(pattern: &Grid, lower: i32, upper: i32) -> bool {
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

fn calculate(input: &mut Input) -> ReturnType {
    // println!("{}", input.to_string());
    let mut result: ReturnType = 0;
    for pattern in &input.patterns {
        let mut found = false;
        for x in 0..pattern[0].len() - 1 {
            // dbg!(is_col_mirrow(&pattern, x as i32, (x + 1) as i32));
            if is_col_mirrow(&pattern, x as i32, (x + 1) as i32) {
                if found {
                    panic!("AAAA");
                }
                result += ((x + 1) * 1) as ReturnType;
                println!("Adding column: {}", x + 1);
                found = true;
                println!("column mirror found! {},{}",x, x+1);
                println!("{}", format_pattern(&pattern));
            }
        }
        for y in 0..(pattern.len() - 1) {
            dbg!(pattern.len(), y);
            // dbg!(is_row_mirrow(&pattern, y as i32, (y + 1) as i32));
            if is_row_mirrow(&pattern, y as i32, (y + 1) as i32) {
                if found {
                    panic!("bbbb");
                }
                result += ((y + 1) * 100) as ReturnType;
                println!("Adding row: {}", (y + 1) * 100);
                found = true;
                println!("column mirror found! {},{}",y, y+1);
                println!("{}", format_pattern(&pattern));
            }
        }
        if !found {
            println!("no match found XXXXXXX");
            println!("{}", format_pattern(pattern));
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
//
//     #[test]
//     fn it_works() {
//         let sample_input = "
// #.##..##.
// ..#.##.#.
// ##......#
// ##......#
// ..#.##.#.
// ..##..##.
// #.#.##.#.
//
// #...##..#
// #....#..#
// ..##..###
// #####.##.
// #####.##.
// ..##..###
// #....#..#
// ";
//         let result = calculate(&mut Input::try_from(sample_input).unwrap());
//         assert_eq!(result, 405);
//     }



    #[test]
    fn it_works2() {
        let sample_input = "
...#...
.##....
##.##..
...#..#
#......
#......
...#..#
##.##..
.##.#..
...#...
.#..#.#
...###.
#..##.#
.##....
..##.##
#.####.
#.####.
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 100);
    }


    //     #[test]
    //     fn it_works2() {
    //         let sample_input = "
    // ...#...####...#..
    // .....##.##.##....
    // ##....######....#
    // ..#.##.#..#.##...
    // ##.###.####.###.#
    // ..###...##...###.
    // #####.##..##.####
    // #######....######
    // ###...#.##.#...##
    // ....###.##.###...
    // ##.####.##.####.#
    // ..###...##...###.
    // ##.#.##....##.#.#
    // ##..#.#....#.#..#
    // ##.###.#..#.###.#
    // ###.#...##...#.##
    // ..####.####.####.
    // ";
    //         let result = calculate(&mut Input::try_from(sample_input).unwrap());
    //         assert_eq!(result, 405);
    //     }
}
