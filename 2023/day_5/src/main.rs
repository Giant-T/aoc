#![allow(dead_code)]

#[derive(Debug)]
struct ConversionMap {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl ConversionMap {
    pub fn is_in_sources(&self, num: &usize) -> bool {
        *num >= self.source_start && *num < self.source_start + self.length
    }

    pub fn convert(&self, num: &usize) -> usize {
        self.destination_start + (num - self.source_start)
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut chunks = input.split("\r\n\r\n");

    let seeds: Vec<usize> = chunks
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    let maps: Vec<Vec<ConversionMap>> = chunks
        .map(|chunk| {
            let mut lines = chunk.lines();
            let _mapping = lines
                .next()
                .unwrap()
                .split_once(' ')
                .unwrap()
                .0
                .split_once("-to-")
                .unwrap();

            let conversion_maps = lines
                .map(|line| {
                    let mut nums = line.splitn(3, ' ');
                    ConversionMap {
                        destination_start: nums.next().unwrap().parse().unwrap(),
                        source_start: nums.next().unwrap().parse().unwrap(),
                        length: nums.next().unwrap().parse().unwrap(),
                    }
                })
                .collect();

            return conversion_maps;
        })
        .collect();

    let result = seeds.iter().map(|seed| {
        let mut num = *seed;

        for map in &maps {
            for conversion_map in map {
                if conversion_map.is_in_sources(&num) {
                    num = conversion_map.convert(&num);
                    break;
                }
            }
        }

        num
    }).min().unwrap();

    println!("{seeds:?}");
    println!("{result}");
}
