use std::collections::{HashMap, HashSet};
use std::fs;

fn part_one() {
    let mut antinode_locs: HashSet<(i32, i32)> = HashSet::new();
    let mut antenna_locs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let input = fs::read_to_string("Day 08/input.txt").expect("");
    let input_size = (
        input.lines().count() as i32,
        input.lines().next().expect("").len() as i32,
    );

    for (i, line) in input.lines().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if character == '.' {
                continue;
            }
            antenna_locs
                .entry(character)
                .and_modify(|x| x.push((i as i32, j as i32)))
                .or_insert(vec![(i as i32, j as i32)]);
        }
    }

    for (_antenna_type, antennas) in antenna_locs {
        for (i, antenna_one) in antennas.iter().enumerate() {
            for antenna_two in antennas[i + 1..].iter() {
                for antinode_loc in [
                    (
                        antenna_one.0 + (antenna_one.0 - antenna_two.0),
                        antenna_one.1 + (antenna_one.1 - antenna_two.1),
                    ),
                    (
                        antenna_two.0 - (antenna_one.0 - antenna_two.0),
                        antenna_two.1 - (antenna_one.1 - antenna_two.1),
                    ),
                ] {
                    if !(antinode_loc.0 < 0
                        || antinode_loc.0 >= input_size.0
                        || antinode_loc.1 < 0
                        || antinode_loc.1 >= input_size.1)
                    {
                        antinode_locs.insert((antinode_loc.0 as i32, antinode_loc.1 as i32));
                    }
                }
            }
        }
    }
    println!("{}", antinode_locs.len());
}

fn part_two() {
    let mut antinode_locs: HashSet<(i32, i32)> = HashSet::new();
    let mut antenna_locs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let input = fs::read_to_string("Day 08/input.txt").expect("");
    let input_size = (
        input.lines().count() as i32,
        input.lines().next().expect("").len() as i32,
    );

    for (i, line) in input.lines().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if character == '.' {
                continue;
            }
            antenna_locs
                .entry(character)
                .and_modify(|x| x.push((i as i32, j as i32)))
                .or_insert(vec![(i as i32, j as i32)]);
        }
    }

    for (_antenna_type, antennas) in antenna_locs {
        for (i, antenna_one) in antennas.iter().enumerate() {
            for antenna_two in antennas[i + 1..].iter() {
                antinode_locs.insert(*antenna_one);
                antinode_locs.insert(*antenna_two);
                // first try all harmonics in one direction
                let mut antinode_loc = (
                    antenna_one.0 + (antenna_one.0 - antenna_two.0),
                    antenna_one.1 + (antenna_one.1 - antenna_two.1),
                );
                while !(antinode_loc.0 < 0
                    || antinode_loc.0 >= input_size.0
                    || antinode_loc.1 < 0
                    || antinode_loc.1 >= input_size.1)
                {
                    antinode_locs.insert((antinode_loc.0 as i32, antinode_loc.1 as i32));
                    antinode_loc = (
                        antinode_loc.0 + (antenna_one.0 - antenna_two.0),
                        antinode_loc.1 + (antenna_one.1 - antenna_two.1),
                    );
                }
                // then the other direction
                antinode_loc = (
                    antenna_two.0 - (antenna_one.0 - antenna_two.0),
                    antenna_two.1 - (antenna_one.1 - antenna_two.1),
                );
                while !(antinode_loc.0 < 0
                    || antinode_loc.0 >= input_size.0
                    || antinode_loc.1 < 0
                    || antinode_loc.1 >= input_size.1)
                {
                    antinode_locs.insert((antinode_loc.0 as i32, antinode_loc.1 as i32));
                    antinode_loc = (
                        antinode_loc.0 - (antenna_one.0 - antenna_two.0),
                        antinode_loc.1 - (antenna_one.1 - antenna_two.1),
                    );
                }
            }
        }
    }
    println!("{}", antinode_locs.len());
}

fn main() {
    part_one();
    part_two();
}
