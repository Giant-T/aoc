#![allow(dead_code)]

use std::str::Split;

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

    pub fn has_num_in_range(&self, range: &Range) -> bool {
        self.is_in_sources(&range.0) || self.is_in_sources(&(range.0 + range.1 - 1))
    }

    pub fn convert_range(&self, range: &Range) -> (Range, Option<Range>) {
        if self.is_in_sources(&range.0) && self.is_in_sources(&(range.0 + range.1 - 1)) {
            // All is in
            ((self.convert(&range.0), range.1), None)
        } else if range.0 < self.source_start {
            // Only upperbound is in
            let new_length = self.source_start - range.0;

            (
                (self.destination_start, range.1 - new_length),
                Some((range.0, new_length)),
            )
        } else {
            // Only lowerbound is in
            let new_start = self.convert(&range.0);
            let new_length = self.length - (range.0 - self.source_start);

            (
                (new_start, new_length),
                Some((self.source_start + self.length, range.1 - new_length)),
            )
        }
    }
}

type Range = (usize, usize);

fn main() {
    let input = include_str!("../input.txt");

    let result = solve_part2(input);

    println!("result: {result}");
}

fn solve_part2(input: &str) -> usize {
    let mut chunks = input.split("\r\n\r\n");

    let seeds: Vec<Range> = parse_seeds(&mut chunks);
    let maps: Vec<Vec<ConversionMap>> = parse_maps(chunks);

    let ranges = find_locations_ranges(&seeds, &maps);

    ranges.iter().flatten().map(|x| x.0).min().unwrap()
}

fn brute_force_part2(input: &str) -> usize {
    let mut chunks = input.split("\r\n\r\n");

    let seeds: Vec<Range> = parse_seeds(&mut chunks);

    let maps: Vec<Vec<ConversionMap>> = parse_maps(chunks);

    *find_locations(&seeds, &maps).iter().min().unwrap()
}

fn find_locations(seeds: &Vec<Range>, maps: &Vec<Vec<ConversionMap>>) -> Vec<usize> {
    seeds
        .iter()
        .map(|range| {
            (range.0..(range.0 + range.1)).map(|seed| {
                let mut num = seed;

                for map in maps {
                    for conversion_map in map {
                        if conversion_map.is_in_sources(&num) {
                            num = conversion_map.convert(&num);
                            break;
                        }
                    }
                }

                num
            })
        })
        .flatten()
        .collect()
}

fn find_locations_ranges(seeds: &Vec<Range>, maps: &Vec<Vec<ConversionMap>>) -> Vec<Vec<Range>> {
    seeds
        .iter()
        .map(|range| {
            let mut ranges = vec![*range];

            for map in maps {
                let length = ranges.len();

                for i in 0..length {
                    let mut current_range = ranges[i];

                    for conversion_map in map {
                        if conversion_map.has_num_in_range(&current_range) {
                            let (new, old) = conversion_map.convert_range(&current_range);

                            if let Some(new_range) = old {
                                ranges.push(new);
                                current_range = new_range;
                            } else {
                                current_range = new;
                                break;
                            }
                        }
                    }

                    ranges[i] = current_range;
                }
            }

            ranges
        })
        .collect()
}

fn parse_seeds(chunks: &mut Split<'_, &str>) -> Vec<Range> {
    chunks
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<usize>>()
        .chunks_exact(2)
        .map(|range| (range[0], range[1]))
        .collect()
}

fn parse_maps(chunks: Split<'_, &str>) -> Vec<Vec<ConversionMap>> {
    chunks
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
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{
        brute_force_part2, find_locations, find_locations_ranges, parse_maps, parse_seeds,
        solve_part2,
    };

    #[test]
    fn sample() {
        let sample = include_str!("../sample.txt");

        let result = solve_part2(sample);

        assert_eq!(46, result);
    }

    #[test]
    fn sample_bruteforce() {
        let sample = include_str!("../sample.txt");

        let result = brute_force_part2(sample);

        assert_eq!(46, result);
    }

    #[test]
    fn brute_force_eq_ranges() {
        let sample = include_str!("../sample.txt");
        let mut chunks = sample.split("\r\n\r\n");

        let seeds = parse_seeds(&mut chunks);
        let maps = parse_maps(chunks);

        let mut brute_force = find_locations(&seeds, &maps);
        brute_force.sort();
        let mut ranges: Vec<usize> = find_locations_ranges(&seeds, &maps)
            .iter()
            .map(|x| {
                x.iter()
                    .map(|(start, length)| Vec::from_iter(*start..(start + length)))
                    .flatten()
            })
            .flatten()
            .collect();

        ranges.sort();

        assert_eq!(brute_force, ranges);
    }
}
