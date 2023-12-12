#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Status {
    Unknown,
    Damaged,
    Working,
}

impl TryFrom<char> for Status {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Status::Damaged),
            '.' => Ok(Status::Working),
            '?' => Ok(Status::Unknown),
            _ => Err("Cannot Parse Tile"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Line {
    statuses: Vec<Status>,
    groups: Vec<usize>,
}

impl TryFrom<&str> for Line {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 2);
        let statuses = parts[0]
            .chars()
            .map(Status::try_from)
            .collect::<Result<Vec<Status>, Self::Error>>()?;
        let groups = parts[1]
            .split(',')
            .map(|n| n.parse::<usize>().map_err(|_err| "Cannot parse value"))
            .collect::<Result<Vec<usize>, Self::Error>>()?;
        let mut unfolded_statuses= Vec::new();
        let mut unfolded_groups = Vec::new();

        for _i in 0..5 {
            unfolded_statuses.extend(&statuses);
            unfolded_statuses.push(Status::Unknown);
            unfolded_groups.extend(&groups);
        }
        Ok(Line { statuses: unfolded_statuses, groups: unfolded_groups })
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for status in &self.statuses {
            result += match status {
                Status::Working => ".",
                Status::Damaged => "#",
                Status::Unknown => "?",
            };
        }
        result += &format!(" {:?}", self.groups);
        result
    }
}

#[derive(Debug)]
struct Input {
    lines: Vec<Line>,
}

impl ToString for Input {
    fn to_string(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl TryFrom<&str> for Input {
    type Error = &'static str;

    fn try_from(input_str: &str) -> Result<Self, Self::Error> {
        let lines = input_str
            .lines()
            .filter(|l| !l.is_empty())
            .map(Line::try_from)
            .collect::<Result<Vec<Line>, Self::Error>>()?;
        Ok(Input { lines })
    }
}

fn get_combs(count: usize) -> Vec<Vec<Status>> {
    if count == 0 {
        return vec![Vec::new()];
    } else {
        let mut result = Vec::new();
        let rest = get_combs(count - 1);
        for mut r in rest {
            let mut a = vec![Status::Working];
            let mut b = vec![Status::Damaged];
            a.extend(&r);
            b.append(&mut r);
            result.push(a);
            result.push(b);
        }
        result
    }
}

fn merge_comb(statuses: &[Status], comb: Vec<Status>) -> Vec<Status> {
    let mut result = Vec::new();
    let mut comb_it = comb.into_iter();
    for s in statuses {
        if *s == Status::Unknown {
            result.push(comb_it.next().unwrap())
        } else {
            result.push(s.clone());
        }
    }
    result
}

fn is_match(statuses: &[Status], groups: &[usize]) -> bool {
    let mut counts: Vec<usize> = Vec::new();
    let mut current_count = 0;
    for s in statuses {
        if *s == Status::Working {
            if current_count > 0 {
                counts.push(current_count);
                current_count = 0;
            }
        } else {
            current_count += 1;
        }
    }
    if current_count > 0 {
        counts.push(current_count);
    }
    groups.len() == (&counts).len() && groups.iter().zip(counts).all(|(a,b)|*a==b)
}

fn match_count(statuses: &[Status], groups: &[usize]) -> u128 {
    let unknown_count = statuses
        .iter()
        .map(|s| if *s == Status::Unknown { 1 } else { 0 })
        .sum();
    println!("getting unknown combs");
    let unknown_combs = get_combs(unknown_count);
    println!("getting unknown combs...done");
    // let combs: Vec<Vec<Status>> = unknown_combs
    //     .into_iter()
    //     .map(|u| merge_comb(statuses, u))
    //     .collect();
    //
    // //dbg!(statuses, groups, unknown_count, combs);
    // combs
    //     .into_iter()
    //     .map(|s| if is_match(&s, groups) { 1 } else { 0 })
    //     .sum()

    unknown_combs
        .into_iter()
        .map(|u| merge_comb(statuses, u))
        .map(|s| if is_match(&s, groups) { 1 } else { 0 })
        .sum()
}
fn get_match_count(line: &Line) -> u128 {
    let result = match_count(&line.statuses, &line.groups);
    // dbg!(&line, &result);
    result
}
fn calculate(input: &mut Input) -> u128 {
    println!("{}", input.to_string());
    input.lines.iter().map(get_match_count).sum()
}

pub fn run() {
    let input_str = include_str!("../input.txt");
    let mut input = Input::try_from(input_str).unwrap();
    let result = calculate(&mut input);
    println!("Result for day11a: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 525152);
    }

    #[test]
    fn it_works2() {
        let sample_input = "
???.### 1,1,3
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works3() {
        let sample_input = "
.??..??...?##. 1,1,3
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 16384);
    }

    #[test]
    fn it_works4() {
        let sample_input = "
?#?#?#?#?#?#?#? 1,3,1,6
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works5() {
        let sample_input = "
????.#...#... 4,1,1
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 16);
    }

    #[test]
    fn it_works6() {
        let sample_input = "
????.######..#####. 1,6,5
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 2500);
    }

    #[test]
    fn it_works7() {
        let sample_input = "
?###???????? 3,2,1
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 506250);
    }

}
