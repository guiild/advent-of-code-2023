use std::cmp::Ordering;
use std::{collections::HashMap, fs};

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    hand_type: u32,
}

impl Hand {
    fn new(cards: Vec<char>, bid: u32, hand_type: u32) -> Hand {
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            return Ordering::Greater;
        }
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        }
        let self_strength = self
            .cards
            .iter()
            .map(|&card| get_strength(card))
            .collect::<Vec<u32>>();
        let other_strength = other
            .cards
            .iter()
            .map(|&card| get_strength(card))
            .collect::<Vec<u32>>();
        for i in 0..5 {
            if self_strength[i] > other_strength[i] {
                return Ordering::Greater;
            }
            if self_strength[i] < other_strength[i] {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (self.hand_type, &self.cards) == (other.hand_type, &other.cards)
    }
}

impl Eq for Hand {}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let mut hands = data
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let cards = split.next().unwrap().chars().collect::<Vec<char>>();
            let bid = split.next().unwrap().parse::<u32>().unwrap();
            let hand_type = get_hand_type(&cards);
            Hand::new(cards, bid, hand_type)
        })
        .collect::<Vec<Hand>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            println!("{}: {:?}", i, hand);
            hand.bid * (i as u32 + 1)
        })
        .sum()
}

fn get_hand_type(cards: &[char]) -> u32 {
    let card_count = cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card).or_insert(0) += 1;
        acc
    });
    // Five of a kind, where all five cards have the same label: AAAAA
    if card_count.keys().len() == 1 {
        return 7;
    };
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    if card_count.values().any(|&count| count == 4) {
        return 6;
    };
    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    if card_count.values().any(|&count| count == 3) && card_count.values().any(|&count| count == 2)
    {
        return 5;
    };
    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    if card_count.values().any(|&count| count == 3) {
        return 4;
    };
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    if card_count.values().filter(|&&count| count == 2).count() == 2 {
        return 3;
    };
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    if card_count.values().filter(|&&count| count == 2).count() == 1 {
        return 2;
    };
    // High card, where all cards' labels are distinct: 23456
    if !card_count.keys().len() == 5 {
        panic!("At this point we should have 5 different cards");
    };
    1
}

fn get_strength(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(6440, solve_puzzle("../test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(249483956, solve_puzzle("../input"));
    }
}
