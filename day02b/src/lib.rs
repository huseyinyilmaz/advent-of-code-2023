use std::cmp;

#[derive(Debug, Default)]
struct Turn {
    red: i32,
    green: i32,
    blue: i32,
}




#[derive(Debug)]
struct Game {
    id: i32,
    turns: Vec<Turn>,
}

fn get_lines(input: &str) -> Vec<&str> {
    input
        .split('\n')
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .collect()
}

fn parse_line(line: &str) -> Game {
    let (title_str, rest_title_str) = line.split_once(':').unwrap();
    let game_id_str = &title_str[5..];
    let game_id = game_id_str.parse().unwrap();
    let mut turns = Vec::new();
    for turn_str_raw in rest_title_str.split(';') {
        let turn_str = turn_str_raw.trim();
        let mut turn = Turn::default();
        for color_str_raw in turn_str.split(',') {
            let color_str = color_str_raw.trim();
            let (color_count, color_name) = color_str.split_once(' ').unwrap();
            turn = match color_name {
                "blue" => Turn {
                blue: color_count.parse().unwrap(),
                    ..turn
            },
                "green" => Turn {
                green: color_count.parse().unwrap(),
                    ..turn
            },

                "red" => Turn {
                red: color_count.parse().unwrap(),
                    ..turn
            },
                _ => panic!()
            }
                
        }
        turns.push(turn);
    }

    Game { id: game_id, turns }

}

fn calculate_result(game: Game) -> i32 {
    let mut green = 0; 
    let mut red = 0;
    let mut blue = 0;
    for turn in game.turns {
        green = cmp::max(green, turn.green);
        red = cmp::max(red, turn.red);
        blue = cmp::max(blue, turn.blue);
    }
    green * red * blue
}

fn calculate(input: &str) -> i32 {
    get_lines(input)
        .into_iter()
        .map(parse_line)
        .map(calculate_result)
        .sum()
}


pub fn run() {
    let input_str = include_str!("../input.txt");
    let result = calculate(input_str);
    println!("Result for day02b: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let result = calculate(sample_input);
        assert_eq!(result, 2286);
    }
}
