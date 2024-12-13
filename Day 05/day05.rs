use std::{collections::HashMap, fs};

fn part_one() {
    let input = fs::read_to_string("Day 05/input.txt").expect("");
    let mut rules: HashMap<&str,Vec<&str>> = HashMap::new();

    let mut sum_of_middle_page_numbers = 0;
    for line in input.lines() {
        println!("{}", line);
        if line.contains("|") {
            rules.entry(line.split("|").nth(1).expect("")).and_modify(|e| e.push(line.split("|").nth(0).expect(""))).or_insert(vec![line.split("|").nth(0).expect("")]);
            continue;
        }

        if line == "" {
            continue;
        }
        
        let mut prev_numbers: Vec<&str> = Vec::new();
        let mut is_correct = true;
        for number in line.split(",") {
            if rules.contains_key(number) {
                for req in rules.get(number).expect("") {
                    if !prev_numbers.contains(req) && 
                        line.split(",").into_iter().any(|i| i == *req) {
                    is_correct = false;
                    break
                }
            } 
           }
            prev_numbers.push(number);
        }
        if is_correct {
            sum_of_middle_page_numbers += line.split(",").nth((line.split(",").count() + 1)/2 -1).expect("").parse::<usize>().expect("");
        }
    }
   println!("{}", sum_of_middle_page_numbers)
}

fn part_two() {

    
}
fn main() {
part_one();
part_two()}