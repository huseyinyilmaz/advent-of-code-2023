use std::fmt::Display;

type ReturnType = u128;

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

impl Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} - {})", self.label, self.focal_length)
    }
}

#[derive(Debug)]
enum Operation {
    Remove(String),
    Add(Lens),
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            Self::Remove(label) => format!("Remove({})", label),
            Self::Add(lens) => format!("Add({})", lens),
        };
        write!(f, "{}", op_str)
    }
}

impl TryFrom<&str> for Operation {
    type Error = &'static str;

    fn try_from(input_str: &str) -> Result<Self, Self::Error> {
        let input_chars: Vec<char> = input_str.chars().collect();
        let label: String = input_chars
            .iter()
            .take_while(|c| !['=', '-'].contains(c))
            .collect();
        let operation = match &input_str[label.len()..label.len() + 1] {
            "-" => Operation::Remove(label),
            "=" => {
                let focal_length = input_str[label.len() + 1..]
                    .parse()
                    .map_err(|_| "Cannot parse focal length")?;
                Operation::Add(Lens {
                    label,
                    focal_length,
                })
            }
            &_ => panic!("Cannot parse operation"),
        };
        dbg!(&input_str, &operation);
        Ok(operation)
    }
}

#[derive(Debug)]
struct Input {
    operations: Vec<Operation>,
}

impl TryFrom<&str> for Input {
    type Error = &'static str;

    fn try_from(input_str: &str) -> Result<Self, Self::Error> {
        let mut operations = Vec::new();
        for line in input_str.lines() {
            if !line.is_empty() {
                let line_ops = line
                    .split(",")
                    .map(Operation::try_from)
                    .collect::<Result<Vec<Operation>, Self::Error>>()?;
                operations.extend(line_ops);
            }
        }
        Ok(Input { operations })
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Operations {}",
            self.operations
                .iter()
                .map(|o| format!("{}", o))
                .collect::<String>()
        )
    }
}

fn hash_word(word: &String) -> usize {
    let mut result = 0;
    for c in word.chars() {
        // Determine the ASCII code for the current character of the string.
        let code = c as usize;
        // Increase the current value by the ASCII code you just determined.
        result += code;
        // Set the current value to itself multiplied by 17.
        result *= 17;
        // Set the current value to the remainder of dividing itself by 256.
        result %= 256;
    }
    println!("{} => {}", word, result);
    result
}

#[derive(Debug)]
struct Storage {
    boxes: Vec<Vec<Lens>>,
}
impl Storage {
    fn calculate(&self) -> ReturnType {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_idx, b)| {
                b.iter()
                    .enumerate()
                    .map(|(lens_idx, lens)| {
                        (box_idx + 1) as ReturnType
                            * (lens_idx + 1) as ReturnType
                            * lens.focal_length as ReturnType
                    })
                    .sum::<ReturnType>()
            })
            .sum()
    }
}

impl From<&Input> for Storage {
    fn from(input: &Input) -> Self {
        let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| Vec::new()).collect();

        for op in &input.operations {
            match &op {
                Operation::Remove(label) => {
                    let box_idx = hash_word(&label);
                    boxes[box_idx].retain_mut(|lens| lens.label != *label);
                    dbg!(op, box_idx);
                }
                Operation::Add(lens) => {
                    let box_idx = hash_word(&lens.label);
                    match boxes[box_idx].iter().position(|l| lens.label == l.label) {
                        Some(lens_idx) => boxes[box_idx][lens_idx] = lens.clone(),
                        None => boxes[box_idx].push(lens.clone()),
                    }

                    dbg!(op, box_idx);
                }
            }
        }
        Self { boxes }
    }
}
fn calculate(input: &Input) -> ReturnType {
    println!("{}", input);
    let storage = Storage::from(input);
    dbg!(&storage);
    storage.calculate()
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
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
        let result = calculate(&mut Input::try_from(sample_input).unwrap());
        assert_eq!(result, 145);
    }
}
