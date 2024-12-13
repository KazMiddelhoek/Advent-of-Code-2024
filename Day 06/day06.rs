use std::collections::HashSet;
use std::fs;

fn get_new_pos(current_pos: (usize, usize), current_dir: &str) -> (usize, usize) {
    let mut new_pos = current_pos.clone(); 
    if current_dir == "up" {
            if new_pos.0 == 0 {return new_pos}
            new_pos.0 -= 1;
        } else if current_dir == "down" {
            new_pos.0 += 1;
        } else if current_dir == "left" {
            if new_pos.1 == 0 {return new_pos}
            new_pos.1 -= 1;
            
        } else if current_dir == "right" {
            new_pos.1 +=1;
        }
    return new_pos
}

fn get_new_dir(mut current_dir: &str) -> &str {
    if current_dir == "up" {
            current_dir = "right";
        } else if current_dir == "down" {
            current_dir = "left";
        } else if current_dir == "left" {
            current_dir = "up";
        } else if current_dir == "right" {
            current_dir = "down";
        }
    return current_dir
}

fn part_one() {
    let input = fs::read_to_string("Day 06/input.txt").expect("").lines().map(|line| {line.chars().collect::<Vec<char>>()}).collect::<Vec<Vec<char>>>();
    let mut current_pos = ( 36 as usize, 81 as usize);
    let mut current_dir = "up";
    let mut visited_pos : HashSet<(usize, usize)> = HashSet::new();

    loop {
        println!("{:?}", current_pos);
        visited_pos.insert(current_pos);
        let new_pos = get_new_pos(current_pos, current_dir);
        if  current_pos == new_pos || new_pos.0 >= input.len() || new_pos.1 >= input[0].len() {
            break
        }

        if input[new_pos.0][new_pos.1] == '#' {
            current_dir = get_new_dir(current_dir);
            continue;
        }
        current_pos = new_pos;
    }
    println!("{}",visited_pos.len())
}

fn part_two() {
}

fn main() {
part_one();
part_two()}