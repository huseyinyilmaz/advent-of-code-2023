use std::collections::HashMap;

type ReturnType = u128;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Rounded,
    Cube,
    Empty,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Cube),
            'O' => Ok(Tile::Rounded),
            '.' => Ok(Tile::Empty),
            _ => Err("Cannot Parse Tile"),
        }
    }
}

type Platform = Vec<Vec<Tile>>;

#[derive(Debug)]
struct Input {
    platform: Platform,
}

fn format_platform(platform: &Platform) -> String {
    let mut result = String::new();
    for line in platform {
        for tile in line {
            result += match tile {
                Tile::Cube => "#",
                Tile::Rounded => "O",
                Tile::Empty => ".",
            };
        }
        result += "\n";
    }
    result += "\n";
    result
}

impl ToString for Input {
    fn to_string(&self) -> String {
        format_platform(&self.platform)
    }
}

impl TryFrom<&str> for Input {
    type Error = &'static str;

    fn try_from(input_str: &str) -> Result<Self, Self::Error> {
        let mut platform = Vec::new();
        for line in input_str.lines() {
            if !line.is_empty() {
                // Line is not empty
                let platform_line = line
                    .chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<Tile>, Self::Error>>()?;
                platform.push(platform_line);
            }
        }
        Ok(Input { platform })
    }
}

fn shift_tile_north(platform: &mut Platform, x: usize, y: usize) {
    let mut current = y;
    while current > 0 && platform[current - 1][x] == Tile::Empty {
        (platform[current][x], platform[current - 1][x]) =
            (platform[current - 1][x], platform[current][x]);
        current -= 1;
    }
}

fn tilt_north(platform: &mut Platform) {
    for x in 0..platform[0].len() {
        for y in 0..platform.len() {
            if platform[y][x] == Tile::Rounded {
                shift_tile_north(platform, x, y);
            }
        }
    }
}

fn shift_tile_south(platform: &mut Platform, x: usize, y: usize) {
    let mut current = y;
    while current < platform.len() - 1 && platform[current + 1][x] == Tile::Empty {
        (platform[current][x], platform[current + 1][x]) =
            (platform[current + 1][x], platform[current][x]);
        current += 1;
    }
}
fn tilt_south(platform: &mut Platform) {
    for x in 0..platform[0].len() {
        for y in (0..platform.len()).rev() {
            if platform[y][x] == Tile::Rounded {
                shift_tile_south(platform, x, y);
            }
        }
    }
}

fn shift_tile_west(platform: &mut Platform, x: usize, y: usize) {
    let mut current = x;
    while current > 0 && platform[y][current - 1] == Tile::Empty {
        (platform[y][current], platform[y][current - 1]) =
            (platform[y][current - 1], platform[y][current]);
        current -= 1;
    }
}

fn tilt_west(platform: &mut Platform) {
    for y in 0..platform.len() {
        for x in 0..platform[y].len() {
            if platform[y][x] == Tile::Rounded {
                shift_tile_west(platform, x, y);
            }
        }
    }
}

fn shift_tile_east(platform: &mut Platform, x: usize, y: usize) {
    let mut current = x;
    while current < platform[0].len() - 1 && platform[y][current + 1] == Tile::Empty {
        (platform[y][current], platform[y][current + 1]) =
            (platform[y][current + 1], platform[y][current]);
        current += 1;
    }
}

fn tilt_east(platform: &mut Platform) {
    for y in 0..platform.len() {
        for x in (0..platform[y].len()).rev() {
            if platform[y][x] == Tile::Rounded {
                shift_tile_east(platform, x, y);
            }
        }
    }
}

fn cycle(platform: &mut Platform) {
    tilt_north(platform);
    tilt_west(platform);
    tilt_south(platform);
    tilt_east(platform);
}

fn calculate(input: &mut Input) -> ReturnType {
    println!("Input");
    println!("{}", input.to_string());
    let mut result: ReturnType = 0;
    let mut p = input.platform.clone();
    tilt_north(&mut p);
    // for i in 0..1000000000 {
    //     if i % 10000000 == 0 {
    //         println!("{}%", (i as f32 / 1000000000.0) * 100.0);
    //     }
    //     cycle(&mut p);
    //     let key = format_platform(&p);
    //     if nodes.contains_key(&key) {
    //         panic!("Repeated")
    //     } else {
    //         nodes.insert(key, p.clone()); 
    //     }
    // }
    // println!("Tilted");
    // println!("{}", format_platform(&p));
    for y in 0..p.len() {
        for x in 0..p[y].len() {
            if p[y][x] == Tile::Rounded {
                result += (p.len() - y) as ReturnType;
            }
        }
    }
    result
}

pub fn run() {
    let input_str = include_str!("../input.txt");
    let mut input = Input::try_from(input_str).unwrap();
    let result = calculate(&mut input);
    println!("Result for day13a: {}", result);
}
// 42361
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 136);
    }
}
