struct Part {
    num: String,
    pub is_part: bool,
}

impl Part {
    pub fn new() -> Self {
        Self {
            num: String::new(),
            is_part: false,
        }
    }

    pub fn push(&mut self, ch: char) {
        self.num.push(ch);
    }

    pub fn clear(&mut self) {
        self.num.clear();
        self.is_part = false;
    }

    pub fn get_num(&mut self) -> u32 {
        self.num.parse().unwrap()
    }
}

fn is_special_char(ch: &u8) -> bool {
    *ch != b'.' && !ch.is_ascii_digit()
}

fn main() {
    let input = include_str!("../sample.txt");

    let lines = input
        .lines()
        .collect::<Vec<_>>();

    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        let mut part = Part::new();
        for (x, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                part.push(char);

                if let Some(minus_y) = y.checked_sub(1) {
                    let previous_line = lines[minus_y].as_bytes();
                    for i in 0..3 {
                        if let Some(index) = (x + i).checked_sub(1) {
                            if let Some(char) = previous_line.get(index) {
                                part.is_part |= is_special_char(char);
                            }
                        }
                    }
                }

                if let Some(index) = x.checked_sub(1) {
                    let prev_char = line.as_bytes()[index];
                    part.is_part |= is_special_char(&prev_char);
                }
                if let Some(next_char) = line.as_bytes().get(x + 1) {
                    part.is_part |= is_special_char(next_char);
                }

                if let Some(next_line) = lines.get(y + 1) {
                    for i in 0..3 {
                        let next_line = next_line.as_bytes();
                        if let Some(index) = (x + i).checked_sub(1) {
                            if let Some(char) = next_line.get(index) {
                                part.is_part |= is_special_char(char);
                            }
                        }
                    }
                }
            } else {
                if part.is_part {
                    sum += part.get_num();
                }
                part.clear();
            }
        }
    }

    println!("{sum}");
}
