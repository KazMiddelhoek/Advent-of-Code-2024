use std::fs;
use regex::Regex;

fn part_one() {
    let mut sum_of_muls = 0;

    let input = fs::read_to_string("Day 03/input.txt").expect("");
    for line in input.lines() {
        {
            let re = Regex::new(r"mul\(\d+?,\d+?\)").unwrap();
            for mul in re.find_iter(line) {
                let first_num = mul.as_str().split(",").collect::<Vec<&str>>()[0];
                let first_num_int = first_num[4..first_num.len()].parse::<i32>().unwrap();
 
                let second_num = mul.as_str().split(",").collect::<Vec<&str>>()[1];
                let second_num_int = second_num[0..second_num.len()-1].parse::<i32>().unwrap();

                sum_of_muls += first_num_int * second_num_int;
            }
        } 
    }
    println!("{}", sum_of_muls);
}

fn part_two() {
    let mut sum_of_muls = 0;

    let input = fs::read_to_string("Day 03/input.txt").expect("");
    let mut prev_instruction: Option<&str> = None;

    for line in input.lines() {
        {
            let re = Regex::new(r"mul\(\d+?,\d+?\)|do\(\)|don't\(\)").unwrap();

            for mul in re.find_iter(line) {
                if prev_instruction == Some("don't()") && mul.as_str().contains("mul") {
                    continue;
                }
                if mul.as_str().contains("mul") {
                    let first_num = mul.as_str().split(",").collect::<Vec<&str>>()[0];
                    let first_num_int = first_num[4..first_num.len()].parse::<i32>().unwrap();
    
                    let second_num = mul.as_str().split(",").collect::<Vec<&str>>()[1];
                    let second_num_int = second_num[0..second_num.len()-1].parse::<i32>().unwrap();
                    sum_of_muls += first_num_int * second_num_int;
                }
                
                if ["do()", "don't()"].contains(&mul.as_str()) { 
                    prev_instruction = Some(mul.as_str());
                }
            }
        } 
    }
    println!("{}", sum_of_muls);
}

fn main() {
    part_one();
    part_two();
}