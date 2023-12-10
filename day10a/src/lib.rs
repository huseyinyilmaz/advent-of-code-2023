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
                x: location.x -1,
                y: location.y,
            };
            match self.get_tile(&candidate) {
                Tile::NorthEast | Tile::EastWest| Tile::EastSouth => Some(candidate),
                _ => None,
            }
        }
    }

    fn get_east(&self, location: &Location) -> Option<Location> {
        if location.y == self.grid[0].len() - 1 {
            None
        } else {

            let candidate = Location {
                x: location.x +1,
                y: location.y,
            };
            match self.get_tile(&candidate) {
                Tile::NorthWest | Tile::EastWest| Tile::WestSouth => Some(candidate),
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
    let mut steps = 0;
    seen.insert(start_location.clone());
    frontier.push(start_location);
    while !frontier.is_empty() {
        let mut new_frontier = Vec::new();
        for current in frontier {
            for next in input.get_next_locations(&current) {
                if !seen.contains(&next) {
                    seen.insert(next.clone());
                    new_frontier.push(next);
                }
            }
        }

        if new_frontier.is_empty() {
            return steps;
        } else {
            steps += 1;
            frontier = new_frontier;
        }
    }
    steps
}

pub fn run() {
    let input_str = include_str!("../input.txt");

    let input = Input::from(input_str);
    let result = calculate(input);
    println!("Result for day10a: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        let result = calculate(Input::from(sample_input));
        assert_eq!(result, 4);
    }
}
