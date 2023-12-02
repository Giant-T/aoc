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
            _ => Err(())
        }
    }
}

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

    let games = input.lines().map(|line| {
        let game = line.split_once(": ").unwrap();

        let id: u32 = game.0.split_once(' ').unwrap().1.parse().unwrap();

        return (id, game.1);
    });

    let result: u32 = games.filter_map(|game| {
        let game_result = game.1.split("; ").find(|x| {
            return x.split(", ").find(|y| {
                let nb_color = y.split_once(' ').unwrap();

                return Color::from_str(nb_color.1).unwrap().max() < nb_color.0.parse::<u32>().unwrap();
            }).is_some();
        });

        if game_result.is_some() {
            None
        } else {
            Some(game.0)
        }
    }).sum();

    println!("{result}");
}
