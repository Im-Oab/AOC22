use std::time::Instant;

use hashbrown::HashMap;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::file_handler::FileHandler;

pub struct Day07 {}

impl Day07 {
    pub fn run() -> (String, String, String, u128, u128) {
        let first_input = FileHandler::read("./src/Y2023/inputs/day_07_1.txt");

        let lines: Vec<&str> = first_input.split('\n').collect();

        let start_1 = Instant::now();
        let result_1 = Day07::part_01(&lines);
        let time_calculation_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let result_2 = Day07::part_02(&lines);
        let time_calculation_2 = start_2.elapsed();

        (
            "Day_07".to_owned(),
            format!("{}", result_1),
            format!("{}", result_2),
            time_calculation_1.as_nanos(),
            time_calculation_2.as_nanos(),
        )
    }

    fn part_01(lines: &Vec<&str>) -> u128 {
        let mut hands = parse_input(lines);
        hands.sort();

        hands
            .par_iter()
            .enumerate()
            .map(|(index, hand)| (index + 1) as u128 * hand.bet)
            .sum()
    }

    fn part_02(lines: &Vec<&str>) -> u128 {
        let mut hands = parse_input_with_joker(lines);
        hands.sort();

        hands
            .par_iter()
            .enumerate()
            .map(|(index, hand)| (index + 1) as u128 * hand.bet)
            .sum()
    }
}

fn parse_input(lines: &Vec<&str>) -> Vec<Hand> {
    lines
        .iter()
        .map(|input| {
            let values: Vec<&str> = input.split(" ").collect();
            let bet = values[1].parse::<u128>().unwrap();
            let hand = Hand::new(parse_cards(values[0]), bet);

            hand
        })
        .collect()
}

fn parse_input_with_joker(lines: &Vec<&str>) -> Vec<Hand> {
    lines
        .iter()
        .map(|input| {
            let values: Vec<&str> = input.split(" ").collect();
            let bet = values[1].parse::<u128>().unwrap();
            let hand = Hand::new(parse_cards_with_joker(values[0]), bet);

            hand
        })
        .collect()
}

fn get_card_value(letter: &str) -> i32 {
    match letter {
        "T" => 10,
        "J" => 11,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        "J" => 0,
        _ => letter.parse::<i32>().unwrap_or(0),
    }
}

fn get_card_value_with_joker(letter: &str) -> i32 {
    match letter {
        "T" => 10,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        "J" => 0,
        _ => letter.parse::<i32>().unwrap_or(0),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPairs = 3,
    OnePairs = 2,
    HighCard = 1,
}

impl HandType {
    pub fn from_cards(cards: &Vec<i32>) -> Self {
        let mut new_card = cards.clone();
        new_card.sort();

        let mut counting = HashMap::new();
        for card in new_card.iter() {
            if let Some(value) = counting.get_mut(card) {
                *value += 1;
            } else {
                counting.insert(*card, 1);
            }
        }

        // enhance
        let j = 0;
        if let Some(total_j) = counting.get(&j).cloned() {
            counting.remove(&j);
            let max_total_cards =
                if let Some((key, total_cards)) = counting.iter().max_by_key(|(_, value)| *value) {
                    *total_cards
                } else {
                    0
                };

            if counting.len() > 0 {
                let mut max_cards: Vec<i32> = counting
                    .iter()
                    .filter_map(|(card, totals)| (*totals == max_total_cards).then_some(*card))
                    .collect();
                max_cards.sort();
                if let Some(card_key) = max_cards.last() {
                    if let Some(value) = counting.get_mut(card_key) {
                        *value += total_j;
                    }
                }
            } else {
                // Transform into `Ace`
                let a = 14;
                counting.insert(a, total_j);
            }
        }

        if counting.len() == 1 {
            return HandType::FiveOfKind;
        }
        // FourKind, FullHouse
        else if counting.len() == 2 {
            for (_, value) in counting.iter() {
                if *value == 4 {
                    return HandType::FourOfKind;
                }
            }

            return HandType::FullHouse;
        }
        // ThreeKind, TwoPairs,
        else if counting.len() == 3 {
            for (_, value) in counting.iter() {
                if *value == 3 {
                    return HandType::ThreeOfKind;
                }
            }

            return HandType::TwoPairs;
        } else if counting.len() == 4 {
            return HandType::OnePairs;
        }

        HandType::HighCard
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Hand {
    pub cards: Vec<i32>,
    pub hand_type: HandType,
    pub bet: u128,
}

impl Hand {
    fn new(cards: Vec<i32>, bet: u128) -> Self {
        let hand_type = HandType::from_cards(&cards);
        Self {
            cards: cards,
            hand_type: hand_type,
            bet: bet,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(std::cmp::Ordering::Equal) => {
                for index in 0..self.cards.len() {
                    match self.cards[index].partial_cmp(&other.cards[index]) {
                        Some(core::cmp::Ordering::Equal) => {}
                        ord => return ord,
                    }
                }
            }
            ord => return ord,
        }

        self.hand_type.partial_cmp(&other.hand_type)
    }
}

fn parse_cards(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| get_card_value(&String::from(c)))
        .collect()
}

fn parse_cards_with_joker(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| get_card_value_with_joker(&String::from(c)))
        .collect()
}

const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test_hand_type_from_card() {
        let cards = [
            "AAAAA", "AA8AA", "23332", "TTT98", "23432", "A23A4", "23456",
        ];
        let expected_results = [
            HandType::FiveOfKind,
            HandType::FourOfKind,
            HandType::FullHouse,
            HandType::ThreeOfKind,
            HandType::TwoPairs,
            HandType::OnePairs,
            HandType::HighCard,
        ];

        for index in 0..cards.len() {
            let input = parse_cards(cards[index]);
            assert_eq!(expected_results[index], HandType::from_cards(&input));
        }
    }

    #[test]
    fn test_hand_type_from_card_with_j() {
        let cards = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"];
        let expected_results = [
            HandType::OnePairs,
            HandType::FourOfKind,
            HandType::TwoPairs,
            HandType::FourOfKind,
            HandType::FourOfKind,
        ];

        for index in 0..cards.len() {
            let input = parse_cards_with_joker(cards[index]);
            assert_eq!(expected_results[index], HandType::from_cards(&input));
        }
    }

    #[test]
    fn test_compare_hand_type() {
        assert!(HandType::FiveOfKind > HandType::FourOfKind);
        assert!(HandType::FiveOfKind > HandType::HighCard);
        assert!(!(HandType::HighCard > HandType::FiveOfKind));

        let weak_hand = Hand::new(parse_cards("T55J5"), 0);
        let strong_hand = Hand::new(parse_cards("QQQJA"), 0);

        assert_eq!(weak_hand.partial_cmp(&strong_hand).unwrap(), Ordering::Less);
        assert_eq!(
            weak_hand.partial_cmp(&weak_hand.clone()).unwrap(),
            Ordering::Equal
        );
        assert_eq!(
            strong_hand.partial_cmp(&weak_hand).unwrap(),
            Ordering::Greater
        );
    }

    #[test]
    fn test_sort_hands() {
        let middle_hand = Hand::new(parse_cards("T55J5"), 0);
        let weak_hand = Hand::new(parse_cards("KK677"), 0);
        let strong_hand = Hand::new(parse_cards("QQQJA"), 0);

        let mut list = vec![strong_hand.clone(), weak_hand.clone(), middle_hand.clone()];
        list.sort();
        assert_eq!(list[0], weak_hand);
        assert_eq!(list[1], middle_hand);
        assert_eq!(list[2], strong_hand);
    }

    #[test]
    fn test_part_1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day07::part_01(&lines);
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        let result = Day07::part_02(&lines);
        assert_eq!(result, 5905);
    }
}
