use std::fs;

fn main() {
    let mut total_diff = 0;

    let input = fs::read_to_string("Day 01/input.txt").expect("");
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input.lines() {
        let left = line.split("   ").next();
        let right = line.split("   ").last();
        left_list.push(left.unwrap().parse::<i32>().unwrap());
        right_list.push(right.unwrap().parse::<i32>().unwrap());
    }
    left_list.sort();
    right_list.sort();

    for (left, right) in left_list.iter().zip(right_list.iter()) {
        total_diff += (left - right).abs();
    }
    println!("{}", total_diff);

    // part two
    let mut sum_of_similarity_scores = 0;
    for elem in left_list {
        sum_of_similarity_scores += (right_list.iter().filter(|x| **x == elem).count() as i32) * elem
    }
    println!("{}", sum_of_similarity_scores);
}