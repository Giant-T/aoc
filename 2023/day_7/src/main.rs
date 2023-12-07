#![allow(dead_code)]

use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Hand {
    cards: [char; 5],
    bid: usize,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand = s.trim().split_once(' ').unwrap();
        let cards = hand.0.chars().collect::<Vec<_>>().try_into().unwrap();
        let bid = hand.1.parse().unwrap();

        Ok(Hand { cards, bid })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    pub fn get_type(&self) -> HandType {
        let mut kinds: HashMap<char, u8> = HashMap::new();

        for card in self.cards.iter() {
            let val = kinds.get_mut(card);
            if let Some(val) = val {
                *val += 1;
            } else {
                kinds.insert(*card, 1);
            }
        }

        let mut two_maxes = (0, 0);

        for val in kinds.values() {
            if *val > two_maxes.0 {
                let temp = two_maxes.0;
                two_maxes.0 = *val;

                if temp > two_maxes.1 {
                    two_maxes.1 = temp;
                }
            } else if *val > two_maxes.1 {
                two_maxes.1 = *val;
            }
        }

        match two_maxes {
            (5, 0) => HandType::FiveOfKind,
            (4, _) => HandType::FourOfKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            (_, _) => HandType::HighCard,
        }
    }

    fn cmp2(&self, other: &Hand) -> Ordering {
        let ord = other.get_type2().cmp(&self.get_type2());

        if let Ordering::Equal = ord {
            for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                let ord = get_card_val2(card).cmp(&get_card_val2(&other_card));

                let Ordering::Equal = ord else {
                    return ord;
                };
            }

            Ordering::Equal
        } else {
            ord
        }
    }

    fn get_type2(&self) -> HandType {
        let mut kinds: HashMap<char, u8> = HashMap::new();

        for card in self.cards.iter() {
            let val = kinds.get_mut(card);
            if let Some(val) = val {
                *val += 1;
            } else {
                kinds.insert(*card, 1);
            }
        }

        let mut two_maxes = (0, 0);

        for (_, val) in kinds.iter().filter(|(x, _)| **x != 'J') {
            if *val > two_maxes.0 {
                let temp = two_maxes.0;
                two_maxes.0 = *val;

                if temp > two_maxes.1 {
                    two_maxes.1 = temp;
                }
            } else if *val > two_maxes.1 {
                two_maxes.1 = *val;
            }
        }

        if let Some(val) = kinds.get(&'J') {
            two_maxes.0 += val;
        }

        match two_maxes {
            (5, 0) => HandType::FiveOfKind,
            (4, _) => HandType::FourOfKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            (_, _) => HandType::HighCard,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = other.get_type().cmp(&self.get_type());

        if let Ordering::Equal = ord {
            for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                let ord = get_card_val(card).cmp(&get_card_val(&other_card));

                let Ordering::Equal = ord else {
                    return ord;
                };
            }

            Ordering::Equal
        } else {
            ord
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let result = solve_part2(input);

    println!("result: {result}");
}

fn solve_part1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().filter_map(|line| line.parse().ok()).collect();

    hands.sort_by(|x, y| x.cmp(y));

    calc_total_winnings(hands)
}

fn solve_part2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().filter_map(|line| line.parse().ok()).collect();

    hands.sort_by(|x, y| x.cmp2(y));

    calc_total_winnings(hands)
}

fn calc_total_winnings(hands: Vec<Hand>) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(index, card)| card.bid * (index + 1))
        .sum()
}

fn get_card_val(card: &char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!(),
    }
}

fn get_card_val2(card: &char) -> usize {
    let value = get_card_val(card);

    if value == 11 {
        return 1;
    }

    value
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn sample_part1() {
        let sample = include_str!("../sample.txt");

        let result = solve_part1(sample);

        assert_eq!(6440, result);
    }

    #[test]
    fn sample_part2() {
        let sample = include_str!("../sample.txt");

        let result = solve_part2(sample);

        assert_eq!(5905, result);
    }
}
