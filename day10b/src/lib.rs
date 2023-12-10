use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Start,
    Empty,
    NorthSouth,
    NorthEast,
    NorthWest,
    EastWest,
    EastSouth,
    WestSouth,
}

impl From<char> for Tile {
    fn from(input: char) -> Self {
        match input {
            'S' => Tile::Start,
            '.' => Tile::Empty,
            '|' => Tile::NorthSouth,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '-' => Tile::EastWest,
            'F' => Tile::EastSouth,
            '7' => Tile::WestSouth,
            _ => panic!("Cannot parse all tiles"),
        }
    }
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Input {
    grid: Grid,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum AnalysisTile {
    Out,
    In,
    Pipe,
}

type AnalysisGrid = Vec<Vec<AnalysisTile>>;

#[derive(Debug)]
struct Analysis {
    grid: AnalysisGrid,
}

impl Analysis {
    fn new(input: Input, seen: HashSet<Location>) -> Self {
        let mut grid: AnalysisGrid = Vec::new();

        for y in 0..input.grid.len() {
            let mut row1 = Vec::new();
            let mut row2 = Vec::new();
            let mut row3 = Vec::new();

            for x in 0..input.grid[0].len() {
                let tile = if seen.contains(&Location { x, y }) {
                    input.grid[y][x]
                } else {
                    Tile::Empty
                };
                let col1;
                let col2;
                let col3;
                match tile {
                    Tile::Start => {
                        col1 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::In];
                        col2 = [AnalysisTile::Pipe, AnalysisTile::Pipe, AnalysisTile::Pipe];
                        col3 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::In];
                    }
                    Tile::Empty => {
                        col1 = [AnalysisTile::In, AnalysisTile::In, AnalysisTile::In];
                        col2 = [AnalysisTile::In, AnalysisTile::In, AnalysisTile::In];
                        col3 = [AnalysisTile::In, AnalysisTile::In, AnalysisTile::In];
                    }
                    Tile::NorthSouth => {
                        col1 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::In];
                        col2 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::In];
                        col3 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::In];
                    }
                    Tile::NorthEast => {
                        col1 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::In];
                        col2 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::Pipe];
                        col3 = [AnalysisTile::In, AnalysisTile::In, AnalysisTile::In];
                    }
                    Tile::NorthWest => {
                        col1 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::In];
                        col2 = [AnalysisTile::Pipe, AnalysisTile::Pipe, AnalysisTile::In];
                        col3 = [AnalysisTile::In, AnalysisTile::In, AnalysisTile::In];
                    }
                    Tile::EastWest => {
                        col1 = [AnalysisTile::In, AnalysisTile::In, AnalysisTile::In];
                        col2 = [AnalysisTile::Pipe, AnalysisTile::Pipe, AnalysisTile::Pipe];
                        col3 = [AnalysisTile::In, AnalysisTile::In, AnalysisTile::In];
                    }
                    Tile::EastSouth => {
                        col1 = [AnalysisTile::In, AnalysisTile::In, AnalysisTile::In];
                        col2 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::Pipe];
                        col3 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::In];
                    }
                    Tile::WestSouth => {
                        col1 = [AnalysisTile::In, AnalysisTile::In, AnalysisTile::In];
                        col2 = [AnalysisTile::Pipe, AnalysisTile::Pipe, AnalysisTile::In];
                        col3 = [AnalysisTile::In, AnalysisTile::Pipe, AnalysisTile::In];
                    }
                }
                row1.extend(col1);
                row2.extend(col2);
                row3.extend(col3);
            }
            grid.push(row1);
            grid.push(row2);
            grid.push(row3);
        }

        for y in 0..grid.len() {
            let loc1 = Location { x: 0, y };
            let loc2 = Location {
                x: grid[0].len() - 1,
                y,
            };
            Analysis::flood_fill(&mut grid, loc1);
            Analysis::flood_fill(&mut grid, loc2);
        }

        for x in 0..grid[0].len() {
            let loc1 = Location { x, y: 0 };
            let loc2 = Location {
                x,
                y: grid.len() - 1,
            };
            Analysis::flood_fill(&mut grid, loc1);
            Analysis::flood_fill(&mut grid, loc2);
        }
        Analysis { grid }
    }

    fn count_inner_tiles(&self) -> i32 {
        let grid = &self.grid;
        let mut count_grid: Vec<Vec<i32>> = vec![vec![0;self.grid[0].len()/3];self.grid.len()/3];
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let tile = &grid[y][x];
                if tile == &AnalysisTile::In {
                    count_grid[y/3][x/3] += 1;
                }
            }
        }
        count_grid.iter().map(|row| row.iter().map(|c| if c == &9 {1} else {0}).sum::<i32>()).sum()
    }

    fn print(&self) {
        for row in &self.grid {
            for tile in row {
                match tile {
                    AnalysisTile::Pipe => print!("P"),
                    AnalysisTile::Out => print!("O"),
                    _ => print!("."),
                }
            }
            println!("");
        }
    }

    fn flood_fill(grid: &mut AnalysisGrid, loc: Location) {
        // it loc is outof bounds, quit.
        if loc.x >= grid[0].len() || loc.y >= grid.len() {
            return;
        }
        let tile = &grid[loc.y][loc.x];
        // It tile is already processed, quit.
        if tile != &AnalysisTile::In {
            return;
        }

        grid[loc.y][loc.x] = AnalysisTile::Out;

        // flood fill north
        if loc.y != 0 {
            Analysis::flood_fill(
                grid,
                Location {
                    x: loc.x,
                    y: loc.y - 1,
                },
            );
        }

        // flood fill south
        if loc.y != grid.len() - 1 {
            Analysis::flood_fill(
                grid,
                Location {
                    x: loc.x,
                    y: loc.y + 1,
                },
            );
        }

        // flood fill east
        if loc.x != grid[0].len() - 1 {
            Analysis::flood_fill(
                grid,
                Location {
                    x: loc.x + 1,
                    y: loc.y,
                },
            );
        }

        // flood fill west
        if loc.x != 0 {
            Analysis::flood_fill(
                grid,
                Location {
                    x: loc.x - 1,
                    y: loc.y,
                },
            );
        }
    }
}

impl Input {
    fn get_start_tile_location(&self) -> Location {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                let maybe_x = row.iter().position(|tile| *tile == Tile::Start);
                maybe_x.map(|x| Location { x, y })
            })
            .unwrap()
    }

    fn get_tile(&self, location: &Location) -> Tile {
        self.grid[location.y][location.x]
    }

    fn get_north(&self, location: &Location) -> Option<Location> {
        if location.y == 0 {
            None
        } else {
            let candidate = Location {
                x: location.x,
                y: location.y - 1,
            };
            match self.get_tile(&candidate) {
                Tile::EastSouth | Tile::WestSouth | Tile::NorthSouth => Some(candidate),
                _ => None,
            }
        }
    }

    fn get_south(&self, location: &Location) -> Option<Location> {
        if location.y == self.grid.len() - 1 {
            None
        } else {
            let candidate = Location {
                x: location.x,
                y: location.y + 1,
            };
            match self.get_tile(&candidate) {
                Tile::NorthEast | Tile::NorthWest | Tile::NorthSouth => Some(candidate),
                _ => None,
            }
        }
    }

    fn get_west(&self, location: &Location) -> Option<Location> {
        if location.x == 0 {
            None
        } else {
            let candidate = Location {
                x: location.x - 1,
                y: location.y,
            };
            match self.get_tile(&candidate) {
                Tile::NorthEast | Tile::EastWest | Tile::EastSouth => Some(candidate),
                _ => None,
            }
        }
    }

    fn get_east(&self, location: &Location) -> Option<Location> {
        if location.y == self.grid[0].len() - 1 {
            None
        } else {
            let candidate = Location {
                x: location.x + 1,
                y: location.y,
            };
            match self.get_tile(&candidate) {
                Tile::NorthWest | Tile::EastWest | Tile::WestSouth => Some(candidate),
                _ => None,
            }
        }
    }

    fn get_next_locations(&self, location: &Location) -> Vec<Location> {
        match &self.get_tile(location) {
            Tile::Start => vec![
                self.get_east(&location),
                self.get_west(location),
                self.get_north(location),
                self.get_south(location),
            ]
            .into_iter()
            .filter_map(|x| x)
            .collect(),
            Tile::Empty => vec![],
            Tile::NorthSouth => vec![self.get_north(location), self.get_south(location)]
                .into_iter()
                .filter_map(|x| x)
                .collect(),
            Tile::NorthEast => vec![self.get_east(location), self.get_north(location)]
                .into_iter()
                .filter_map(|x| x)
                .collect(),
            Tile::NorthWest => vec![self.get_west(location), self.get_north(location)]
                .into_iter()
                .filter_map(|x| x)
                .collect(),
            Tile::EastWest => vec![self.get_east(location), self.get_west(location)]
                .into_iter()
                .filter_map(|x| x)
                .collect(),
            Tile::EastSouth => vec![self.get_east(location), self.get_south(location)]
                .into_iter()
                .filter_map(|x| x)
                .collect(),
            Tile::WestSouth => vec![self.get_west(location), self.get_south(location)]
                .into_iter()
                .filter_map(|x| x)
                .collect(),
        }
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

fn calculate(input: Input) -> i32 {
    let start_location = (&input).get_start_tile_location();
    let mut seen: HashSet<Location> = HashSet::new();
    let mut frontier: Vec<Location> = Vec::new();
    seen.insert(start_location.clone());
    frontier.push(start_location);
    while !frontier.is_empty() {
        let mut new_frontier = Vec::new();
        for current in &frontier {
            for next in input.get_next_locations(&current) {
                if !seen.contains(&next) {
                    seen.insert(next.clone());
                    new_frontier.push(next);
                }
            }
        }

        if new_frontier.is_empty() {
            let analysis = Analysis::new(input, seen);
            // analysis.print();
            return analysis.count_inner_tiles();
        } else {
            frontier = new_frontier;
        }
    }
    0
}

pub fn run() {
    let input_str = include_str!("../input.txt");

    let input = Input::from(input_str);
    let result = calculate(input);
    println!("");
    println!("Result for day10b: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        let result = calculate(Input::from(sample_input));
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let sample_input = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let result = calculate(Input::from(sample_input));
        assert_eq!(result, 8);
    }
    #[test]
    fn it_works3() {
        let sample_input = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let result = calculate(Input::from(sample_input));
        assert_eq!(result, 10);
    }
}
