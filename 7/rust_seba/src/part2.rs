use crate::utils::read_data;
use std::cmp::Ordering;
use std::collections::HashMap;

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
        let self_cards = self
            .cards
            .iter()
            .map(|&card| get_strength(card))
            .collect::<Vec<u32>>();
        let other_cards = other
            .cards
            .iter()
            .map(|&card| get_strength(card))
            .collect::<Vec<u32>>();
        for i in 0..5 {
            if self_cards[i] > other_cards[i] {
                return Ordering::Greater;
            }
            if self_cards[i] < other_cards[i] {
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

pub fn solve_puzzle(file_name: &str) -> u32 {
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
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

fn get_hand_type(cards: &[char]) -> u32 {
    let card_count = cards.iter().fold(HashMap::new(), |mut acc, card| {
        let count = acc.entry(card).or_insert(0);
        *count += 1;
        acc
    });
    // Five of a kind, where all five cards have the same label: AAAAA
    if card_count.keys().len() == 1
        || card_count.keys().len() == 2 && card_count.keys().any(|&card| card == &'J')
    {
        return 7;
    };
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    if card_count
        .iter()
        .any(|(c, &count)| c != &&'J' && count + card_count.get(&'J').unwrap_or(&0) == 4)
    {
        return 6;
    };
    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    if is_full_house(&card_count, cards) {
        return 5;
    };

    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    if card_count
        .values()
        .any(|&count| count + card_count.get(&'J').unwrap_or(&0) == 3)
    {
        return 4;
    };
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    if card_count.values().filter(|&&count| count == 2).count() == 2
        || (card_count.values().filter(|&&count| count == 2).count() == 1 && cards.contains(&'J'))
    {
        return 3;
    };
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    if card_count.values().filter(|&&count| count == 2).count() == 1 || cards.contains(&'J') {
        return 2;
    };
    // High card, where all cards' labels are distinct: 23456
    1
}

fn is_full_house(card_count: &HashMap<&char, u32>, _cards: &[char]) -> bool {
    // assert_eq!(4, get_hand_type(&['2', '2', 'J', '1', 'A']));
    let card_3 = card_count.iter().find(|(_, &count)| count == 3);
    let card_2 = card_count.iter().find(|(_, &count)| count == 2);
    let card_j = card_count.iter().find(|(&card, _)| card == &'J');
    if card_3.is_some() && card_2.is_some() {
        return true;
    }
    if card_count.iter().filter(|(_, &count)| count == 2).count() == 2 && card_j.is_some() {
        return true;
    }

    false
}

fn get_strength(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 11,
        _ => card.to_digit(10).unwrap(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(5905, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(252137472, solve_puzzle("input"));
    }

    #[test]
    fn test_get_hand_type_5_kind() {
        assert_eq!(7, get_hand_type(&['A', 'A', 'A', 'A', 'J']));
        assert_eq!(7, get_hand_type(&['A', 'A', 'A', 'J', 'J']));
        assert_eq!(7, get_hand_type(&['A', 'A', 'J', 'J', 'J']));
        assert_eq!(7, get_hand_type(&['A', 'J', 'J', 'J', 'J']));
        assert_eq!(7, get_hand_type(&['J', 'J', 'J', 'J', 'J']));
    }

    #[test]
    fn test_get_hand_type_four_kind() {
        assert_eq!(6, get_hand_type(&['A', 'A', 'A', 'A', '8']));
        assert_eq!(6, get_hand_type(&['A', 'A', 'A', 'J', '8']));
        assert_eq!(6, get_hand_type(&['A', 'A', 'J', 'J', '8']));
        assert_eq!(6, get_hand_type(&['A', 'J', 'J', 'J', '8']));
    }

    #[test]
    fn test_get_hand_type_full_house() {
        assert_eq!(5, get_hand_type(&['2', '3', '3', '2', '3']));
        assert_eq!(5, get_hand_type(&['2', 'J', '3', '2', '3']));
    }

    #[test]
    fn test_get_hand_type_three_of_a_kind() {
        assert_eq!(4, get_hand_type(&['2', '2', '2', '1', 'A']));
        assert_eq!(4, get_hand_type(&['2', '2', '2', '1', 'A']));
        assert_eq!(4, get_hand_type(&['2', '2', 'J', '1', 'A']));
        assert_eq!(4, get_hand_type(&['2', 'J', 'J', '1', 'A']));
    }
}
