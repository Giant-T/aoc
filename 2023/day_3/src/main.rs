fn main() {
    let input = include_str!("../input.txt");

    let lines = input
            .lines()
            .collect::<Vec<_>>();

    let sum: u32 = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let mut num = String::new();
            let mut is_part = false;

            return line
                .char_indices()
                .filter_map(|(x, ch)| {
                    if ch.is_ascii_digit() {
                        num.push(ch);

                        if let Some(py) = y.checked_sub(1) {
                            let prev_line = lines[py];
                            is_part |= check_special_near(prev_line, x);
                        }
                        if let Some(next_line) = lines.get(y + 1) {
                            is_part |= check_special_near(next_line, x);
                        }
                        is_part |= check_special_near(line, x);
                    }

                    if x == (line.len() -1) || !ch.is_ascii_digit() {
                        let result = num.parse::<u32>().ok(); 
                        num.clear();

                        if is_part {
                            is_part = false;
                            return result;
                        }

                        is_part = false;
                    }

                    return None;
                })
                .sum::<u32>();
        })
        .sum();

    println!("{:?}", sum);
}

fn is_special_char(ch: &u8) -> bool {
    *ch != b'.' && !ch.is_ascii_digit()
}

fn check_special_near(line: &str, x: usize) -> bool {
    let mut is_part = false;
    let line = line.as_bytes();

    if let Some(px) = x.checked_sub(1) {
        is_part |= is_special_char(&line[px]);
    }

    is_part |= is_special_char(&line[x]);

    if let Some(ch) = line.get(x + 1) {
        is_part |= is_special_char(ch);
    }

    return is_part;
}

