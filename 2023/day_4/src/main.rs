use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let sum: u32 = input.lines()
        .map(|line| 
            line.split_once(':').unwrap().1.split_once(" | ").unwrap()
        ).map(|(winning_nums, nums)| {
            let winning_nums: HashSet<u32> = winning_nums.split(' ')
                .filter_map(|nums| nums.parse::<u32>().ok())
                .collect();

            let mut score = 0;
            let nums = nums.split(' ').filter_map(|n| n.parse::<u32>().ok());

            for num in nums {
                if let Some(_) = winning_nums.get(&num) {
                    score = (score << 1) | (score == 0) as u32;
                }
            }

            return score;
        }).sum();

    println!("{sum}");
}
