use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    TreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}


#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Hash)]
struct Card {
    value: u32,
    symbol: char,
}

impl From<char> for Card {
    fn from(item: char) -> Self {
        let value = match item {
            item if item.is_digit(10) => item.to_digit(10).unwrap(),
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        };
        return Card {
            symbol: item,
            value,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl From<&str> for Hand {
    fn from(item: &str) -> Self {
        // item = "32T3K 765";
        let cols: Vec<&str> = item.split_whitespace().collect();
        let bid = cols[1].parse().unwrap();
        let cards: Vec<Card> = cols[0].chars().map(Card::from).collect();
        assert!(cards.len() == 5);
        Hand { bid, cards }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_hand_type() == other.get_hand_type() {
            self.cards.cmp(&other.cards)
        } else {
            self.get_hand_type().cmp(&other.get_hand_type())
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.get_hand_type() == other.get_hand_type() {
            self.cards.partial_cmp(&other.cards)
        } else {
            self.get_hand_type().partial_cmp(&other.get_hand_type())
        }
    }
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let mut counts: HashMap<&Card, u32> = HashMap::new();
        for card in &self.cards {
            counts.entry(card).and_modify(|counter| *counter += 1).or_insert(1);
        }
        let mut vals: Vec<u32> = counts.into_values().collect();
        vals.sort();
        match vals[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::TreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => {
                dbg!(&self);
                panic!("Could not get hand_type")
            },
        }
    }
}

#[derive(Debug)]
struct Input {
    hands: Vec<Hand>,
}

impl From<&str> for Input {
    fn from(input_str: &str) -> Self {
        let hands = input_str
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(Hand::from)
            .collect();
        Input { hands }
    }
}

fn calculate(mut input: Input) -> u32 {
    input.hands.sort();
    input.hands.into_iter().enumerate().map(|(idx, hand)| (idx as u32 +1)* hand.bid).sum()
}
pub fn run() {
    let input_str = include_str!("../input.txt");

    let input = Input::from(input_str);
    let result = calculate(input);
    println!("Result for day07a: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_type_ord() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
    }

    #[test]
    fn it_works() {
        let sample_input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let result = calculate(Input::from(sample_input));
        assert_eq!(result, 6440);
    }
}
