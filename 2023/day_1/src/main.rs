fn main() {
    let input = include_str!("../input.txt");

    let num: u32 = input.lines().map(|line| {
        let mut left_most = ' ';
        let mut right_most = ' ';

        for char in line.chars() {
            match char {
                '0'..='9' => {
                    if left_most == ' ' {
                        left_most = char;
                    }
                    right_most = char;
                },
                _ => {}
            }
        }

        return format!("{}{}", left_most, right_most).parse::<u32>().unwrap();
    }).sum();

    println!("{num}");
}
