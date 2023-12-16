use std::fmt::Display;

type ReturnType = u128;

#[derive(Debug)]
struct Input {
    words: Vec<String>,
}

impl TryFrom<&str> for Input {
    type Error = &'static str;

    fn try_from(input_str: &str) -> Result<Self, Self::Error> {
        let mut words = Vec::new();
        for line in input_str.lines() {
            if !line.is_empty() {
                words.extend(line.split(",").map(String::from));
            }
        }
        Ok(Input { words })
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Words {}", self.words.join(", "))
    }
}

fn hash_word(word: &String) -> ReturnType {
    let mut result = 0;
    for c in word.chars() {
        // Determine the ASCII code for the current character of the string.
        let code = c as ReturnType;
        // Increase the current value by the ASCII code you just determined.
        result += code;
        // Set the current value to itself multiplied by 17.
        result *= 17;
        // Set the current value to the remainder of dividing itself by 256.
        result %= 256;
    }
    println!("{} => {}",word, result);
    result
}
fn calculate(input: &mut Input) -> ReturnType {
    println!("{}", input);
    input.words.iter().map(hash_word).sum()
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
        assert_eq!(result, 1320);
    }
}
