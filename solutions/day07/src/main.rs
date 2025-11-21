#![allow(warnings)]

use utils;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::cmp::Ordering;


fn main() {
    let raw_input = utils::lines_from_file("day07");
    let p1: usize = calc_p1(&raw_input);
    let p2: usize = calc_p2(&raw_input);
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}


fn calc_p1(raw_input: &Vec<String>) -> usize {
    // convert inputs into hands
    let mut hands: Vec<Hand> = raw_input.iter().map(|line| Hand::try_from(line).unwrap()).collect();

    // order hands by rank
    hands.sort();

    // compute score
    let mut score: usize = 0;
    for i in 0..hands.len() {
        score += hands[i].bid * (i + 1);
    }
    score
}

fn calc_p2(raw_input: &Vec<String>) -> usize {
    // convert inputs into hands
    let mut hands: Vec<Hand> = raw_input.iter().map(|line| Hand::try_from(line).unwrap()).collect();

    // order hands by rank
    hands.sort();

    // compute score
    let mut score: usize = 0;
    for i in 0..hands.len() {
        score += hands[i].bid * (i + 1);
    }
    score
}


// idea for improvement:
// map jokers onto a separate char and always treat jokers as wild in the hand_rank calculation
// then only map jacks onto jokers in p2

///////////////
// supporting methods
///////////////


#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
enum HandRank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}


#[derive(Debug)]
#[derive(Eq)]
struct Card {
    name: char,
    value: usize
}


// p1
// impl TryFrom<&char> for Card {
//     type Error = &'static str;

//     fn try_from(value: &char) -> Result<Self, Self::Error> {
//         let name = value;
//         let value = match value {
//             'T' => 10,
//             'J' => 11,
//             'Q' => 12,
//             'K' => 13,
//             'A' => 14,
//             _ => value.to_digit(10).unwrap() as usize
//         };
//         Ok(Card { name: *name, value: value })
//     }
// }


// p2
impl TryFrom<&char> for Card {
    type Error = &'static str;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        let name = value;
        let value = match value {
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => value.to_digit(10).unwrap() as usize
        };
        Ok(Card { name: *name, value: value })
    }
}


impl cmp::Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}


impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}


#[derive(Debug)]
#[derive(Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    hand_rank: HandRank,
    hand_rank_value: usize
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_rank_value == other.hand_rank_value {
            self.cards.cmp(&other.cards)
        } else {
            self.hand_rank_value.cmp(&other.hand_rank_value)
        }
    }
}


impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (self.hand_rank_value == other.hand_rank_value) && (self.cards == other.cards)
    }
}


impl TryFrom<&String> for Hand {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.split(" ").collect::<Vec<&str>>().len() != 2 {
            return Err("Input for Hand must have 2 values");
        } else {
            let cards = value
                .split(" ")
                .collect::<Vec<&str>>()[0]
                .chars()
                .collect::<Vec<char>>()
                .iter()
                .map(
                    |c|
                    Card::try_from(c)
                    .unwrap()
                )
                .collect::<Vec<Card>>();
            let bid = value.split(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

            let mut card_counts: HashMap<char, usize> = HashMap::new();
            for card in cards.iter() {
                let count = card_counts.entry(card.name).or_insert(0);
                *count += 1;
            }

            // <p2>

            // treat joker as wild
            let n_jokers: usize = *card_counts.entry('J').or_insert(0);
            if n_jokers == 5 {
                return Ok(Hand { cards, bid, hand_rank: HandRank::FiveOfAKind, hand_rank_value: 6 });
            }
            card_counts.remove(&'J');

            // find which card has the most instances
            let max_card_count = card_counts.values().max().unwrap();
            let max_card_count_key = card_counts.iter().find(|&x| (*x.1 == *max_card_count)).unwrap().0;

            // add n_jokers to card count with largest value
            card_counts.insert(*max_card_count_key, *max_card_count + n_jokers);

            // </p2>

            let hand_rank = match card_counts.values().max() {
                Some(5) => HandRank::FiveOfAKind,
                Some(4) => HandRank::FourOfAKind,
                Some(3) => {
                    if card_counts.values().filter(|&x| *x == 2).count() == 1 {
                        HandRank::FullHouse
                    } else {
                        HandRank::ThreeOfAKind
                    }
                },
                Some(2) => {
                    if card_counts.values().filter(|&x| *x == 2).count() == 2 {
                        HandRank::TwoPair
                    } else {
                        HandRank::OnePair
                    }
                },
                _ => HandRank::HighCard
            };
            let hand_rank_value = match hand_rank {
                HandRank::FiveOfAKind => 6,
                HandRank::FourOfAKind => 5,
                HandRank::FullHouse => 4,
                HandRank::ThreeOfAKind => 3,
                HandRank::TwoPair => 2,
                HandRank::OnePair => 1,
                HandRank::HighCard => 0
            };

            Ok(Hand { cards, bid, hand_rank, hand_rank_value })
        }
    }
}


///////////////
// testing
///////////////

fn get_example_input() -> Vec<String> {
    let test_input: Vec<String> = vec![
        "32T3K 765".to_string(),
        "T55J5 684".to_string(),
        "KK677 28".to_string(),
        "KTJJT 220".to_string(),
        "QQQJA 483".to_string()
    ];
    test_input
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_p1() {
        let example_input = get_example_input();
        assert_eq!(calc_p1(&example_input), 6440);
    }

    #[test]
    fn test_calc_p2() {
        let example_input = get_example_input();
        assert_eq!(calc_p2(&example_input), 5905);
    }
}
