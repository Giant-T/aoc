use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    pub score: usize,
    pub quantity: usize,
}

fn main() {
    let input = include_str!("../input.txt");

    let sum = solve_part2(input);

    println!("{sum}");
}

fn solve_part2(input: &str) -> usize {
    let mut cards: Vec<Card> = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1.split_once(" | ").unwrap())
        .map(|(winning_nums, nums)| {
            let winning_nums: HashSet<u32> = winning_nums
                .split(' ')
                .filter_map(|nums| nums.parse::<u32>().ok())
                .collect();

            let mut score = 0;
            let nums = nums.split(' ').filter_map(|n| n.parse::<u32>().ok());

            for num in nums {
                if let Some(_) = winning_nums.get(&num) {
                    score += 1;
                }
            }

            return Card { score, quantity: 1 };
        })
        .collect();

    for index in 0..cards.len() {
        let card = &cards[index];
        let quantity = card.quantity;
        for offset in 1..=card.score {
            if let Some(n_card) = cards.get_mut(index + offset) {
                n_card.quantity += quantity;
            }
        }
    }

    cards.iter().map(|card| card.quantity).sum()
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap().1.split_once(" | ").unwrap())
        .map(|(winning_nums, nums)| {
            let winning_nums: HashSet<u32> = winning_nums
                .split(' ')
                .filter_map(|nums| nums.parse::<u32>().ok())
                .collect();

            let mut score = 0;
            let nums = nums.split(' ').filter_map(|n| n.parse::<u32>().ok());

            for num in nums {
                if let Some(_) = winning_nums.get(&num) {
                    score = (score << 1) | (score == 0) as usize;
                }
            }

            return score;
        })
        .sum()
}
