use std::str::FromStr;

#[allow(dead_code)]
enum Color {
    Blue,
    Red,
    Green,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Self::Blue),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
impl Color {
    pub fn max(&self) -> u32 {
        match self {
            Color::Blue => 14,
            Color::Red => 12,
            Color::Green => 13,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let result = solve_part2(input);

    println!("{result}");
}

fn solve_part2(input: &str) -> u32 {
    let games = input.lines().map(|line| {
        let game = line.split_once(": ").unwrap();

        let id: u32 = game.0.split_once(' ').unwrap().1.parse().unwrap();

        return (id, game.1);
    });

    games
        .map(|game| {
            let mut red_min = 0;
            let mut blue_min = 0;
            let mut green_min = 0;

            game.1.split("; ").for_each(|x| {
                x.split(", ").for_each(|y| {
                    let nb_color = y.split_once(' ').unwrap();
                    let color = Color::from_str(nb_color.1).unwrap();
                    let nb = nb_color.0.parse::<u32>().unwrap();

                    match color {
                        Color::Blue => {
                            if nb > blue_min {
                                blue_min = nb;
                            }
                        }
                        Color::Red => {
                            if nb > red_min {
                                red_min = nb;
                            }
                        }
                        Color::Green => {
                            if nb > green_min {
                                green_min = nb;
                            }
                        }
                    }
                });
            });

            return red_min * blue_min * green_min;
        })
        .sum()
}

fn solve_part1(input: &str) -> u32 {
    let games = input.lines().map(|line| {
        let game = line.split_once(": ").unwrap();

        let id: u32 = game.0.split_once(' ').unwrap().1.parse().unwrap();

        return (id, game.1);
    });

    games
        .filter_map(|game| {
            let game_result = game.1.split("; ").find(|x| {
                return x
                    .split(", ")
                    .find(|y| {
                        let nb_color = y.split_once(' ').unwrap();

                        return Color::from_str(nb_color.1).unwrap().max()
                            < nb_color.0.parse::<u32>().unwrap();
                    })
                    .is_some();
            });

            if game_result.is_some() {
                None
            } else {
                Some(game.0)
            }
        })
        .sum()
}
