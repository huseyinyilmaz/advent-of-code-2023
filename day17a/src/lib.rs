// 1176 wrong
// 1191 wrong
use std::fmt::Display;

use std::collections::HashMap;

type ReturnType = u128;

type Grid = Vec<Vec<u32>>;

#[derive(Debug)]
struct Input {
    grid: Grid,
}

impl Input {
    fn is_inbound(&self, location: &Location) -> bool {
        0 <= location.y
            && location.y < self.grid.len() as i32
            && 0 <= location.x
            && location.x < self.grid[0].len() as i32
    }

    fn get_next_locations(
        &self,
        paths: &Vec<Location>,
        routes: &Vec<Direction>,
    ) -> Result<Vec<Location>, String> {
        let location = paths
            .last()
            .ok_or("get_next_locations cannot get last element in paths")?;
        let mut next_locations: Vec<Location> = vec![
            Location {
                x: location.x + 1,
                y: location.y,
            },
            Location {
                x: location.x,
                y: location.y + 1,
            },
            Location {
                x: location.x - 1,
                y: location.y,
            },
            Location {
                x: location.x,
                y: location.y - 1,
            },
        ]
        .into_iter()
        .filter(|l| self.is_inbound(l))
        // .filter(|l| paths.len() < 2 || *l != paths[paths.len()-2])
        .filter(|l| !paths.contains(l))
        .collect();

        if routes.len() > 2 {
            let last_idx = routes.len() - 1;
            // if we have last 3 of the directions same, that means we have last 4 elements in same
            // line. so we cannot go straight any more.
            if routes[last_idx - 1] == routes[last_idx] && routes[last_idx - 2] == routes[last_idx]
            {
                let direction = routes[last_idx];
                next_locations = next_locations
                    .into_iter()
                    .filter(|l| direction != Direction::get_direction(location, l).unwrap())
                    .collect();
            }
        }

        Ok(next_locations)
    }

    fn get(&self, location: &Location) -> Result<u32, String> {
        if self.is_inbound(&location) {
            Ok(self.grid[location.y as usize][location.x as usize])
        } else {
            Err("Location given is out of bounds".to_string())
        }
    }
}

impl TryFrom<&str> for Input {
    type Error = &'static str;

    fn try_from(input_str: &str) -> Result<Self, Self::Error> {
        let mut grid = Vec::new();
        for line in input_str.lines() {
            if !line.is_empty() {
                let grid_line = line
                    .chars()
                    .map(|n| n.to_digit(10).ok_or("Cannot parse digit"))
                    .collect::<Result<Vec<u32>, Self::Error>>()?;
                grid.push(grid_line);
            }
        }
        Ok(Input { grid })
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid")?;
        for line in &self.grid {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match &self {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">",
        };
        write!(f, "{}", val)
    }
}

impl Direction {
    fn get_direction(prev_location: &Location, location: &Location) -> Result<Self, String> {
        match (
            (location.x - prev_location.x).signum(),
            (location.y - prev_location.y).signum(),
        ) {
            (1, 0) => Ok(Self::Right),
            (-1, 0) => Ok(Self::Left),
            (0, 1) => Ok(Self::Down),
            (0, -1) => Ok(Self::Up),
            _ => Err("Cannot find direction".to_string()),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
struct Location {
    x: i32,
    y: i32,
}

fn print_path(input: &Input, paths: &Vec<Location>, routes: &Vec<Direction>) {
    let grid = input.grid.clone();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let loc = Location {
                x: x as i32,
                y: y as i32,
            };
            if let Some(idx) = paths.iter().position(|p| *p == loc) {
                if idx < routes.len() {
                    print!("{}", routes[idx]);
                } else {
                    print!("*");
                }
            } else {
                print!("{}", grid[y][x]);
            }
        }

        println!("");
    }
    println!("");
}
type DFSCacheKey = (Location, Location, Location, Location);
type DFSCache = HashMap<DFSCacheKey, Option<ReturnType>>;

fn get_cache_key(paths: &Vec<Location>) -> Option<DFSCacheKey> {
    let mut key = None;
    if paths.len() > 3 {
        let last_idx = paths.len() - 1;
        let key_val = (
            paths[last_idx - 3],
            paths[last_idx - 2],
            paths[last_idx - 1],
            paths[last_idx],
        );
        key = Some(key_val);
    }
    key
}
fn dfs(
    input: &Input,
    paths: Vec<Location>,
    routes: Vec<Direction>,
    end: &Location,
    mut cache: &mut DFSCache,
) -> Result<Option<ReturnType>, String> {
    // if paths.len() > (input.grid.len() + input.grid[0].len()) * 5 {
    //     // print_path(input, &paths, &routes);
    //     return Ok(None);
    // }
    let start = paths.last().ok_or("Path is empty".to_string())?;
    if start == end {
        print_path(input, &paths, &routes);
        return Ok(Some(0));
    } else {
        let next_locations = input.get_next_locations(&paths, &routes)?;
        // println!("next_locations: {:?}", next_locations);
        // let mut next_results = Vec::new();
        let mut min_result = None;
        for next_location in next_locations {
            let mut new_path = paths.clone();
            let mut new_routes = routes.clone();
            new_path.push(next_location);
            let new_direction = Direction::get_direction(start, &next_location)?;
            new_routes.push(new_direction);
            let next_loss = input.get(&next_location)? as ReturnType;

            let key = get_cache_key(&new_path);
            let maybe_new_result: Option<ReturnType>;
            if key.is_some() && cache.contains_key(&key.unwrap()) {
                maybe_new_result = *cache
                    .get(&key.unwrap())
                    .ok_or("Cache does not have the key")?;
            } else {
                maybe_new_result = dfs(input, new_path, new_routes, end, &mut cache)?;
                if let Some(key_val) = key {
                    cache.insert(key_val, maybe_new_result);
                }
            }

            match maybe_new_result {
                Some(new_result) => {
                    match min_result {
                        Some(min_res) if min_res > new_result + next_loss => {
                            min_result = Some(new_result + next_loss)
                        }
                        None => min_result = Some(new_result + next_loss),
                        _ => {}
                    };
                }
                None => {}
            }
        }
        return Ok(min_result);
    }
}

fn calculate(input: &Input) -> ReturnType {
    println!("{}", input);
    let start = Location { x: 0, y: 0 };
    let end = Location {
        x: (input.grid[0].len() - 1) as i32,
        y: (input.grid.len() - 1) as i32,
    };
    // dijkstra(input, start, end).unwrap()
    dfs(input, vec![start], Vec::new(), &end, &mut HashMap::new())
        .unwrap()
        .unwrap()
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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 102);
    }
}
