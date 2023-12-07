fn main() {
    let input = include_str!("../input.txt");

    let num = solve_part2(input);

    println!("{num}");
}

fn get_first_number(input: &str) -> char {
    if input.chars().next().unwrap().is_ascii_digit() {
        return input.chars().next().unwrap();
    }

    let numbers = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    return numbers
        .iter()
        .find(|x| input.starts_with(x.0))
        .unwrap_or(&("", ' '))
        .1;
}

fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut left_most = ' ';
            let mut right_most = ' ';

            for index in 0..line.len() {
                let remaining_line = &line[index..];

                let char = get_first_number(remaining_line);

                if let '0'..='9' = char {
                    if left_most == ' ' {
                        left_most = char;
                    }
                    right_most = char;
                }
            }

            return format!("{}{}", left_most, right_most)
                .parse::<u32>()
                .unwrap();
        })
        .sum()
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut left_most = ' ';
            let mut right_most = ' ';

            for char in line.chars() {
                match char {
                    '0'..='9' => {
                        if left_most == ' ' {
                            left_most = char;
                        }
                        right_most = char;
                    }
                    _ => {}
                }
            }

            return format!("{}{}", left_most, right_most)
                .parse::<u32>()
                .unwrap();
        })
        .sum()
}
