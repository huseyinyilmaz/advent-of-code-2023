use std::fmt::Display;

use std::collections::HashSet;

type ReturnType = u128;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,        // .
    Vertical,     // |
    Horizontal,   // -
    LeftLeaning,  // \
    RightLeaning, // /
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            Tile::Empty => ".",
            Tile::Vertical => "|",
            Tile::Horizontal => "-",
            Tile::LeftLeaning => "\\",
            Tile::RightLeaning => "/",
        };
        write!(f, "{}", op_str)
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            '\\' => Ok(Self::LeftLeaning),
            '/' => Ok(Self::RightLeaning),
            _ => Err("Cannot Parse Tile"),
        }
    }
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug)]
struct Input {
    grid: Grid,
}
impl Input {
    fn is_inbound(&self, x: i32, y: i32) -> bool {
        0 <= y && y < self.grid.len() as i32 && 0 <= x && x < self.grid[0].len() as i32
    }

    fn get_tile(&self, x: i32, y: i32) -> Tile {
        if self.is_inbound(x, y) {
            self.grid[y as usize][x as usize]
        } else {
            Tile::Empty
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
                    .map(Tile::try_from)
                    .collect::<Result<Vec<Tile>, Self::Error>>()?;
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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Light {
    direction: Direction,
    x: i32,
    y: i32,
}
impl Light {
    fn run(&self, input: &Input) -> Vec<Light> {
        let next_x = match &self.direction {
            &Direction::Left => self.x - 1,
            &Direction::Right => self.x + 1,
            _ => self.x,
        };

        let next_y = match &self.direction {
            &Direction::Up => self.y - 1,
            &Direction::Down => self.y + 1,
            _ => self.y,
        };
        if input.is_inbound(next_x, next_y) {
            let next_directions = match (&self.direction, input.get_tile(next_x, next_y)) {
                (_, Tile::Empty)
                | (Direction::Up | Direction::Down, Tile::Vertical)
                | (Direction::Left | Direction::Right, Tile::Horizontal) => vec![self.direction],
                (Direction::Up | Direction::Down, Tile::Horizontal) => {
                    vec![Direction::Left, Direction::Right]
                }
                (Direction::Left | Direction::Right, Tile::Vertical) => {
                    vec![Direction::Up, Direction::Down]
                }
                (Direction::Up, Tile::LeftLeaning) => vec![Direction::Left],
                (Direction::Up, Tile::RightLeaning) => vec![Direction::Right],
                (Direction::Down, Tile::LeftLeaning) => vec![Direction::Right],
                (Direction::Down, Tile::RightLeaning) => vec![Direction::Left],
                (Direction::Left, Tile::LeftLeaning) => vec![Direction::Up],
                (Direction::Left, Tile::RightLeaning) => vec![Direction::Down],
                (Direction::Right, Tile::LeftLeaning) => vec![Direction::Down],
                (Direction::Right, Tile::RightLeaning) => vec![Direction::Up],
            };

            next_directions
                .into_iter()
                .map(|direction| Light {
                    direction,
                    x: next_x,
                    y: next_y,
                })
                .collect()
        } else {
            // New location is out of bounds
            return Vec::new();
        }
    }
}

fn calculate_light(input: &Input, light: Light) -> ReturnType {
    println!("{}", input);
    let mut lights = vec![light];
    let mut seen: HashSet<Light> = HashSet::new();
    while !(&lights).is_empty() {
        let mut new_lights = Vec::new();
        for light in &lights {
            let light_result: Vec<Light> = light.run(&input).into_iter().filter(|light|!seen.contains(&light)).collect();
            seen.extend(light_result.clone().into_iter());
            new_lights.extend(light_result.into_iter());
        }
        lights = new_lights;
    }
    seen.into_iter().map(|light|(light.x, light.y)).collect::<HashSet<(i32, i32)>>().len() as ReturnType
}


fn calculate(input: &Input) -> ReturnType {
    let top = (0..input.grid[0].len()).map(|x| Light{direction:Direction::Down, x:x as i32, y:-1});
    let bottom = (0..input.grid[0].len()).map(|x| Light{direction:Direction::Up, x:x as i32, y:input.grid.len() as i32});
    let left = (0..input.grid[0].len()).map(|y| Light{direction:Direction::Right, x:-1, y:y as i32});
    let right = (0..input.grid[0].len()).map(|y| Light{direction:Direction::Left, x:input.grid[0].len() as i32, y:y as i32});

    top.chain(bottom).chain(left).chain(right).map(|light| calculate_light(input, light)).max().unwrap()
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
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 51);
    }
}
