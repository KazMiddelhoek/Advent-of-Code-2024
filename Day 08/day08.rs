use std::collections::{HashMap, HashSet};
use std::fs;

fn part_one() {
    let mut antinode_locs: HashSet<(usize, usize)> = HashSet::new();
    let mut antenna_locs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let input = fs::read_to_string("Day 08/input.txt").expect("");
    let input_size = (input.lines().count(), input.lines().next().expect("").len());

    for (i, line) in input.lines().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if character == '.' {
                continue;
            }
            antenna_locs
                .entry(character)
                .and_modify(|x| x.push((i, j)))
                .or_insert(vec![(i, j)]);
        }
    }

    for (_antenna_type, antennas) in antenna_locs {
        for (i, antenna_one) in antennas.iter().enumerate() {
            for antenna_two in antennas[i + 1..].iter() {
                for antinode_loc in [
                    (
                        antenna_one.0 as isize + (antenna_one.0 as isize - antenna_two.0 as isize),
                        antenna_one.1 as isize + (antenna_one.1 as isize - antenna_two.1 as isize),
                    ),
                    (
                        antenna_two.0 as isize - (antenna_one.0 as isize - antenna_two.0 as isize),
                        antenna_two.1 as isize - (antenna_one.1 as isize - antenna_two.1 as isize),
                    ),
                ] {
                    if !(antinode_loc.0 < 0
                        || antinode_loc.0 >= input_size.0 as isize
                        || antinode_loc.1 < 0
                        || antinode_loc.1 >= input_size.1 as isize)
                    {
                        antinode_locs.insert((antinode_loc.0 as usize, antinode_loc.1 as usize));
                    }
                }
            }
        }
    }
    println!("{}", antinode_locs.len());
}

fn part_two() {
    let mut antinode_locs: HashSet<(usize, usize)> = HashSet::new();
    let mut antenna_locs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let input = fs::read_to_string("Day 08/input.txt").expect("");
    let input_size = (input.lines().count(), input.lines().next().expect("").len());

    for (i, line) in input.lines().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if character == '.' {
                continue;
            }
            antenna_locs
                .entry(character)
                .and_modify(|x| x.push((i, j)))
                .or_insert(vec![(i, j)]);
        }
    }

    for (_antenna_type, antennas) in antenna_locs {
        for (i, antenna_one) in antennas.iter().enumerate() {
            for antenna_two in antennas[i + 1..].iter() {
                antinode_locs.insert(*antenna_one);
                antinode_locs.insert(*antenna_two);
                // first try all harmonics in one direction
                let mut antinode_loc = (
                    antenna_one.0 as isize + (antenna_one.0 as isize - antenna_two.0 as isize),
                    antenna_one.1 as isize + (antenna_one.1 as isize - antenna_two.1 as isize),
                );
                while !(antinode_loc.0 < 0
                    || antinode_loc.0 >= input_size.0 as isize
                    || antinode_loc.1 < 0
                    || antinode_loc.1 >= input_size.1 as isize)
                {
                    antinode_locs.insert((antinode_loc.0 as usize, antinode_loc.1 as usize));
                    antinode_loc = (
                        antinode_loc.0 as isize + (antenna_one.0 as isize - antenna_two.0 as isize),
                        antinode_loc.1 as isize + (antenna_one.1 as isize - antenna_two.1 as isize),
                    );
                }
                // then the other direction
                antinode_loc = (
                    antenna_two.0 as isize - (antenna_one.0 as isize - antenna_two.0 as isize),
                    antenna_two.1 as isize - (antenna_one.1 as isize - antenna_two.1 as isize),
                );
                while !(antinode_loc.0 < 0
                    || antinode_loc.0 >= input_size.0 as isize
                    || antinode_loc.1 < 0
                    || antinode_loc.1 >= input_size.1 as isize)
                {
                    antinode_locs.insert((antinode_loc.0 as usize, antinode_loc.1 as usize));
                    antinode_loc = (
                        antinode_loc.0 as isize - (antenna_one.0 as isize - antenna_two.0 as isize),
                        antinode_loc.1 as isize - (antenna_one.1 as isize - antenna_two.1 as isize),
                    );
                }
            }
        }
    }
    println!("{:?}", antinode_locs);
    println!("{}", antinode_locs.len());
}

fn main() {
    part_one();
    part_two();
}
