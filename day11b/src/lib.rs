use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Galaxy,
}


impl TryFrom<char> for Tile {

    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Galaxy),
            '.' => Ok(Tile::Empty),
            _ => Err("Cannot Parse Tile"),
        }
    }
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Input {
    grid: Grid,
}

impl Input {
    fn get_tile(&self, location: &Location) -> Tile {
        self.grid[location.y][location.x]
    }

    fn combinations<T: Copy>(&self, vec: &Vec<T>) -> Vec<(T, T)> {
        let mut result = Vec::new();
        for a in 0..vec.len() {
            for b in a + 1..vec.len() {
                result.push((vec[a].clone(), vec[b].clone()));
            }
        }
        return result;
    }

    fn find_galaxies(&self) -> Vec<Location> {
        let mut result = Vec::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let location = Location { x, y };
                if self.get_tile(&location) == Tile::Galaxy {
                    result.push(location);
                }
            }
        }
        return result;
    }

    fn print(&self) {
        for row in &self.grid {
            for tile in row {
                match tile {
                    Tile::Galaxy => print!("#"),
                    _ => print!("."),
                }
            }
            println!("");
        }
        println!("");
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for y in 0..self.grid.len() {
            let mut is_empty = true;
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] != Tile::Empty {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                result.push(y);
            }
        }
        result
    }

    fn get_empty_columns(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for x in 0..self.grid[0].len() {
            let mut is_empty = true;
            for y in 0..self.grid.len() {
                if self.grid[y][x] != Tile::Empty {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                result.push(x);
            }
        }
        result
    }
}

impl TryFrom<&str> for Input {

    type Error = &'static str;

    fn try_from(input_str: &str) -> Result<Self, Self::Error> {
        let grid = input_str
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(Tile::try_from).collect())
            .collect::<Result<Grid,Self::Error>>()?;
        Ok(Input { grid })
    }
}

fn calculate(input: &mut Input, expansion_factor: u128) -> u128 {
    input.print();
    let galaxies = input.find_galaxies();
    let empty_rows = input.get_empty_rows();
    let empty_cols = input.get_empty_columns();
    let combinations = input.combinations(&galaxies);
    // dbg!(&empty_rows, &empty_cols);
    let mut result: u128 = 0;
    for (a, b) in combinations {
        let lower_x = std::cmp::min(a.x, b.x);
        let upper_x = std::cmp::max(a.x, b.x);
        let lower_y = std::cmp::min(a.y, b.y);
        let upper_y = std::cmp::max(a.y, b.y);
        let empty_rows_passed: u128 = (&empty_rows)
            .into_iter()
            .filter(|&row_y| lower_y < *row_y && *row_y < upper_y)
            .map(|_| 1)
            .sum();
        let empty_cols_passed: u128 = (&empty_cols)
            .into_iter()
            .filter(|&row_x| lower_x < *row_x && *row_x < upper_x)
            .map(|_| 1)
            .sum();
        // dbg!(a,b,lower_x,upper_x,lower_y,upper_y,empty_rows_passed);
        result += (upper_x - lower_x) as u128;
        result += (empty_cols_passed * expansion_factor) - empty_cols_passed;
        result += (upper_y - lower_y) as u128;
        result += (empty_rows_passed * expansion_factor) - empty_rows_passed;
    }
    result
}

pub fn run() {
    let input_str = include_str!("../input.txt");
    let mut input = Input::try_from(input_str).unwrap();
    let result = calculate(&mut input, 1000000);
    println!("Result for day11a: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap(), 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn it_works2() {
        let sample_input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap(), 100);
        assert_eq!(result, 8410);
    }
}
