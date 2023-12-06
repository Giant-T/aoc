#![allow(dead_code)]

#[derive(Debug)]
struct Race {
    pub time: u32,
    pub distance: u32,
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);

    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn solve_part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok());

    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok());

    let races: Vec<Race> = times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();

    races
        .iter()
        .map(|race| {
            let mut successes = 0;

            for i in 0..race.time {
                let time_left = race.time - i;
                let distance_traveled = time_left * i;
                if distance_traveled > race.distance {
                    successes += 1;
                }
            }

            successes
        })
        .product()
}

fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();

    let time: usize = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse()
        .unwrap();

    let distance: usize = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse()
        .unwrap();

    let mut possibilities = 0;

    for i in (0..time).rev() {
        let time_left = time - i;
        let distance_traveled = time_left * i;

        if distance_traveled > distance {
            possibilities = i;
            break;
        }
    }

    for i in 1..possibilities {
        let time_left = time - i;
        let distance_traveled = time_left * i;

        if distance_traveled > distance {
            possibilities -= i - 1;
            break;
        }
    }

    possibilities
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn sample_part1() {
        let input = include_str!("../sample.txt");
        let result = solve_part1(input);

        assert_eq!(288, result);
    }

    #[test]
    fn sample_part2() {
        let input = include_str!("../sample.txt");
        let result = solve_part2(input);

        assert_eq!(71503, result);
    }
}
