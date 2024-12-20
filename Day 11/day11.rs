use std::{collections::{HashMap, HashSet}, fs};

fn part_one_and_two() {
    let mut stones = fs::read_to_string("Day 11/input.txt")
        .expect("")
        .lines()
        .next()
        .expect("")
        .split_whitespace()
        .map(|x| (x.to_string(), 1))
        .collect::<HashMap<String, i64>>();

    for _i in 0..75 {
        let mut new_stones = HashMap::<String, i64>::new();
        for (stone, n) in stones {
            if stone == "0" {
                new_stones.entry("1".to_string()).and_modify(|x| *x+=n).or_insert(n);
                continue;
            }

            if stone.len() % 2 == 0 {
                new_stones.entry(stone[0..stone.len() / 2].to_string()).and_modify(|x| *x+=n).or_insert(n);

                let mut new_stone = stone[stone.len() / 2..].to_string();
                while new_stone.starts_with('0') && new_stone.len() > 1 {
                    new_stone = new_stone[1..].to_string();
                }
                new_stones.entry(new_stone).and_modify(|x| *x+=n).or_insert(n);
                continue;
            }
            let new_stone = (stone.parse::<i64>().expect("") * 2024).to_string();
            new_stones.entry(new_stone).and_modify(|x| *x+=n).or_insert(n);
        }
        stones = new_stones;
    }
    println!("{}", stones.values().sum::<i64>());
}

fn main() {
    part_one_and_two();
}
