use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, time};

const INPUT: &str = include_str!("../day1.txt");

fn part_one(input: &str) -> u32 {
    let mut l_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut r_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();

    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(l_num, r_num)| {
            l_heap.push(Reverse(l_num.parse().unwrap()));
            r_heap.push(Reverse(r_num.parse().unwrap()));
        });

    let mut res: u32 = 0;
    while let (Some(l_num), Some(r_num)) = (l_heap.pop(), r_heap.pop()) {
        if l_num.0 > r_num.0 {
            res += l_num.0 - r_num.0
        } else {
            res += r_num.0 - l_num.0
        }
    }
    return res;
}

fn part_two(input: &str) -> u32 {
    let mut l_map: HashMap<u32, u32> = HashMap::new();
    let mut r_map: HashMap<u32, u32> = HashMap::new();

    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(l_num, r_num)| (l_num.parse().unwrap(), r_num.parse().unwrap()))
        .for_each(|(l_num, r_num)| {
            if l_map.contains_key(&l_num) {
                *l_map.get_mut(&l_num).unwrap() += 1;
            } else {
                l_map.insert(l_num, 1);
            }
            if r_map.contains_key(&r_num) {
                *r_map.get_mut(&r_num).unwrap() += 1;
            } else {
                r_map.insert(r_num, 1);
            }
        });

    let res = l_map.iter()
        .map(|(l_num, l_count)| {
            if !r_map.contains_key(&l_num) {
                return 0;
            }

            return l_num * l_count * r_map.get(&l_num).unwrap();
        }).sum();

    return res;
}

fn main() {
    let time = time::Instant::now();
    let res = part_two(INPUT);

    println!("{res}, time: {:?}", time.elapsed());
}
