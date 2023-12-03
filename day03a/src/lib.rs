use std::collections::HashSet;

type Board = Vec<Vec<char>>;


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Number {
    row: usize,
    start_column: usize,
    end_column:usize,
    value: i32,
}

fn get_lines(input: &str) -> Vec<&str> {
    input
        .split('\n')
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .collect()
}

fn get_char(board: &Board, x_input: i32, y_input: i32) -> char {
    if x_input < 0 || y_input < 0 {
        return '.';
    } 

    let x = x_input as usize;
    let y = y_input as usize;

    if y >= board.len() {
        '.'
    } else {
        let row = &board[y];
        if x >= row.len() {
            '.'
        } else {
            board[y][x]
        }
    }
}

fn parse_number(board: &Board, x: usize, y: usize) -> Number {
    let row = &board[y];
    let mut start = x;
    let mut end = x;
    // extend start
    while start > 0 && row[start-1].is_digit(10) {
        start -= 1;
    }

    // extend end
    while end < row.len()-1 && row[end+1].is_digit(10) {
        end += 1;
    }

    let number_str: String = row[start..=end].into_iter().collect();

    Number {
        row:y,
        start_column:start,
        end_column:end,
        value: number_str.parse().unwrap(),
    }
    
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn is_in_engine(board: &Board, number: &Number) -> bool {
    let y = number.row;
    for x in number.start_column..=number.end_column {
        if is_symbol(get_char(&board, x as i32 +1, y as i32  ))
        || is_symbol(get_char(&board, x as i32 +1, y as i32+1))
        || is_symbol(get_char(&board, x as i32 +1, y as i32-1))
        || is_symbol(get_char(&board, x as i32 -1, y as i32  ))
        || is_symbol(get_char(&board, x as i32 -1, y as i32+1))
        || is_symbol(get_char(&board, x as i32 -1, y as i32-1))
        || is_symbol(get_char(&board, x as i32   , y as i32+1))
        || is_symbol(get_char(&board, x as i32   , y as i32-1)) {
            return true;
        }
    }
    false
}

fn find_numbers(board: Board) -> i32 {
    let mut result = 0;
   let mut seen: HashSet<Number> = HashSet::new();
   for row_idx in 0..board.len() {
        let row_len = board[row_idx].len();
        for col_idx in 0..row_len {
            if get_char(&board,col_idx as i32, row_idx as i32).is_digit(10) {
                let number = parse_number(&board, col_idx, row_idx);
                if !seen.contains(&number) {
                    seen.insert(number.clone());
                    if is_in_engine(&board, &number) {
                        result += number.value;
                    }
                }
            }
        }
   }
   result 
}

fn calculate(input: &str) -> i32 {
    let board: Vec<Vec<char>> = get_lines(input)
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    find_numbers(board)
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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let result = calculate(sample_input);
        assert_eq!(result, 4361);
    }
}
