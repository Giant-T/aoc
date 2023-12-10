fn main() {
    let input = include_str!("../input");

    let part1 = solve_part1(input);
    let part2 = solve_part2(input);

    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn solve_part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<isize> = parse_nums(line);

            let mut diffs: Vec<isize> = calc_diffs(&numbers);
            let mut last_val = *numbers.last().unwrap();

            while !diffs.iter().all(|x| *x == 0) {
                last_val += *diffs.last().unwrap();
                diffs = calc_diffs(&diffs);
            }

            last_val
        })
        .sum()
}

fn solve_part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<isize> = parse_nums(line);

            let mut diffs = vec![calc_diffs(&numbers)];

            while !diffs.last().unwrap().iter().all(|x| *x == 0) {
                diffs.push(calc_diffs(&diffs.last().unwrap()));
            }

            let mut last_val = 0;
            for diff in diffs.iter().rev() {
                last_val = diff[0] - last_val;
            }

            numbers[0] - last_val
        })
        .sum()
}

fn parse_nums(line: &str) -> Vec<isize> {
    line.split(' ').filter_map(|x| x.parse().ok()).collect()
}

fn calc_diffs(nums: &Vec<isize>) -> Vec<isize> {
    nums.windows(2).map(|x| x[1] - x[0]).collect()
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn sample_part1() {
        let input = include_str!("../sample");

        let result = solve_part1(input);

        assert_eq!(114, result);
    }

    #[test]
    fn sample_part2() {
        let input = include_str!("../sample");

        let result = solve_part2(input);

        assert_eq!(2, result);
    }
}
