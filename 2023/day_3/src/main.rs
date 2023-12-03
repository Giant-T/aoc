use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let lines = input.lines().collect::<Vec<_>>();

    let mut gear_map = HashMap::<(usize, usize), Vec<u32>>::new();

    lines.iter().enumerate().for_each(|(y, line)| {
        let mut num = String::new();
        let mut gear: Option<(usize, usize)> = None;

        line.char_indices().for_each(|(x, ch)| {
            if ch.is_ascii_digit() {
                num.push(ch);

                if let None = gear {
                    gear = find_nearby_gear(&lines, (x, y));
                }
            }

            if x == (line.len() - 1) || !ch.is_ascii_digit() {
                let result = num.parse::<u32>().ok();
                num.clear();

                if let Some(coords) = gear {
                    if let Some(vec) = gear_map.get_mut(&coords) {
                        vec.push(result.unwrap());
                    } else {
                        gear_map.insert(coords, vec![result.unwrap()]);
                    }
                }

                gear = None;
            }
        });
    });

    let sum: u32 = gear_map.iter().filter_map(|(_, v)| {
        if v.len() == 2 {
            return Some(v[0] * v[1]);
        }

        return None;
    }).sum();

    println!("{sum}");
}

#[allow(dead_code)]
fn is_special_char(ch: &u8) -> bool {
    *ch != b'.' && !ch.is_ascii_digit()
}

#[allow(dead_code)]
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

fn find_nearby_gear(lines: &Vec<&str>, coords: (usize, usize)) -> Option<(usize, usize)> {
    if let Some(py) = coords.1.checked_sub(1) {
        if let Some(x) = find_gear_on_line(lines[py], coords.0) {
            return Some((x, py));
        }
    }

    if let Some(x) = find_gear_on_line(lines[coords.1], coords.0) {
        return Some((x, coords.1));
    }

    if let Some(n_line) = lines.get(coords.1 + 1) {
        if let Some(x) = find_gear_on_line(n_line, coords.0) {
            return Some((x, coords.1 + 1));
        }
    }

    return None;
}

fn find_gear_on_line(line: &str, x: usize) -> Option<usize> {
    let line = line.as_bytes();

    if let Some(px) = x.checked_sub(1) {
        if line[px] == b'*' {
            return Some(px);
        }
    }

    if line[x] == b'*' {
        return Some(x);
    }

    if let Some(ch) = line.get(x + 1) {
        if *ch == b'*' {
            return Some(x + 1);
        }
    }

    return None;
}
