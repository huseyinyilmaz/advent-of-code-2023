#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Galaxy,
}

impl From<char> for Tile {
    fn from(input: char) -> Self {
        match input {
            '#' => Tile::Galaxy,
            '.' => Tile::Empty,
            _ => panic!("Cannot parse all tiles"),
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

    fn combinations<T: Copy>(&self, vec: &Vec<T>) -> Vec<(T,T)> {
        let mut result = Vec::new();
        for a in 0..vec.len() {
           for b in a+1..vec.len() {
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

    fn expand_universe(&mut self) {
        let extend_row = |grid: &Grid| {
            grid.iter()
                .map(|row| {
                    if (&row).into_iter().all(|tile| *tile == Tile::Empty) {
                        vec![row.clone(), row.clone()]
                    } else {
                        vec![row.clone()]
                    }
                })
                .flatten()
                .collect::<Grid>()
        };

        let inverse = |grid: &Grid| {
            let mut result: Vec<Vec<Tile>> = Vec::new();
            for x in 0..grid[0].len() {
                let mut row: Vec<Tile> = Vec::new();
                for y in 0..grid.len() {
                    row.push(grid[y][x]);
                }
                result.push(row);
            }
            return result;
        };
        self.grid = inverse(&extend_row(&inverse(&extend_row(&self.grid))));
        // println!("row extended grid: {:?}", row_extended_grid);
    }
}

impl From<&str> for Input {
    fn from(input_str: &str) -> Self {
        let grid = input_str
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(Tile::from).collect())
            .collect();
        Input { grid }
    }
}

fn calculate(input: &mut Input) -> u32 {
    input.expand_universe();
    input.print();
    let galaxies = input.find_galaxies();
    let combinations = input.combinations(&galaxies);
    combinations.iter().map(|(a, b)| (a.x as u32).abs_diff(b.x as u32) + (a.y as u32).abs_diff(b.y as u32)).sum()
}

pub fn run() {
    let input_str = include_str!("../input.txt");

    let mut input = Input::from(input_str);
    let result = calculate(&mut input);
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
        let result = calculate(&mut Input::from(sample_input));
        assert_eq!(result, 374);
    }
}
