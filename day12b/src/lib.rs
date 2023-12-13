use std::collections::HashMap;

type ReturnType = u128;

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
        let mut unfolded_statuses = Vec::new();
        let mut unfolded_groups = Vec::new();

        for i in 0..5 {
            unfolded_statuses.extend(&statuses);
            if i != 4 {
                unfolded_statuses.push(Status::Unknown);
            }
            unfolded_groups.extend(&groups);
        }
        Ok(Line {
            statuses: unfolded_statuses,
            groups: unfolded_groups,
        })
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

fn count_matches(
    statuses: &[Status],
    groups: &[usize],
    cache: &mut HashMap<String, ReturnType>,
) -> ReturnType {
    //dbg!(statuses, groups);
    // is status is empty finish recursion.
    if statuses.is_empty() {
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    //If value is in cache return it from cache.
    let cache_key = format!("{}-{}", statuses.len(), groups.len());
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let mut result = 0;

    // Case where first element is not part of the first group
    if [Status::Working, Status::Unknown].contains(&statuses[0]) {
        result += count_matches(&statuses[1..], groups, cache);
    }

    //Case where first element is part of the group
    if !groups.is_empty() {
        let group = groups[0];
        if statuses.len() >= group
            && statuses[..group]
                .iter()
                .all(|s| [Status::Damaged, Status::Unknown].contains(s))
        {
            if statuses.len() == group {
                result += count_matches(&statuses[group..], &groups[1..], cache);
            } else if statuses.len() > group
                && [Status::Working, Status::Unknown].contains(&statuses[group])
            {
                // Next element after the group is valid
                // Do not send the next element to recursive call so we are removing it with the group.
                result += count_matches(&statuses[group + 1..], &groups[1..], cache);
            }
        }
    }

    cache.insert(cache_key, result);
    result
}

fn get_match_count(line: &Line) -> ReturnType {
    let result = count_matches(&line.statuses, &line.groups, &mut HashMap::new());
    // dbg!(&line, &result);
    result
}
fn calculate(input: &mut Input) -> ReturnType {
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

    //     #[test]
    //     fn it_works() {
    //         let sample_input = "
    // ???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1
    // ";
    //         let result = calculate(&mut Input::try_from(sample_input).unwrap());
    //         assert_eq!(result, 525152);
    //     }
    //
    //     #[test]
    //     fn it_works2() {
    //         let sample_input = "
    // ???.### 1,1,3
    // ";
    //         let result = calculate(&mut Input::try_from(sample_input).unwrap());
    //         assert_eq!(result, 1);
    //     }
    //
    //     #[test]
    //     fn it_works3() {
    //         let sample_input = "
    // .??..??...?##. 1,1,3
    // ";
    //         let result = calculate(&mut Input::try_from(sample_input).unwrap());
    //         assert_eq!(result, 16384);
    //     }
    //
    //     #[test]
    //     fn it_works4() {
    //         let sample_input = "
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ";
    //         let result = calculate(&mut Input::try_from(sample_input).unwrap());
    //         assert_eq!(result, 1);
    //     }
    //
    //     #[test]
    //     fn it_works5() {
    //         let sample_input = "
    // ????.#...#... 4,1,1
    // ";
    //         let result = calculate(&mut Input::try_from(sample_input).unwrap());
    //         assert_eq!(result, 16);
    //     }
    //
    //     #[test]
    //     fn it_works6() {
    //         let sample_input = "
    // ????.######..#####. 1,6,5
    // ";
    //         let result = calculate(&mut Input::try_from(sample_input).unwrap());
    //         assert_eq!(result, 2500);
    //     }

    #[test]
    fn it_works7() {
        let sample_input = "
?###???????? 3,2,1
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 506250);
    }

//     #[test]
//     fn input_parsing_test() {
//         let sample_input = "
// .# 1
// ";
//         let input = Input::try_from(sample_input).unwrap();
//         println!("{:?}", input);
//         assert_eq!(
//             input.lines,
//             vec![Line {
//                 statuses: vec![
//                     Status::Working,
//                     Status::Damaged,
//                     Status::Unknown,
//                     Status::Working,
//                     Status::Damaged,
//                     Status::Unknown,
//                     Status::Working,
//                     Status::Damaged,
//                     Status::Unknown,
//                     Status::Working,
//                     Status::Damaged,
//                     Status::Unknown,
//                     Status::Working,
//                     Status::Damaged
//                 ],
//                 groups: vec![1, 1, 1, 1, 1]
//             }]
//         );
//     }
}
