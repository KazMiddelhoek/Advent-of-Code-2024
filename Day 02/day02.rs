use std::fs;

fn line_is_safe(levels: &Vec<&str>) -> bool {
    let mut is_increasing:  Option<bool> = None;
    let mut is_safe = true;
    let mut next_level_iter = levels.iter();
    next_level_iter.next();
    for (level, next_level) in levels.clone().iter().zip(next_level_iter) {
        let level_int = level.parse::<i32>().unwrap();
        let next_level_int = next_level.parse::<i32>().unwrap();
        
        if (level_int - next_level_int).abs() < 1 || (level_int - next_level_int).abs() > 3 {
            is_safe = false;
            break;
        }
        if is_increasing != Some(level_int < next_level_int)  && is_increasing != None {
            is_safe = false;
            break
        };
        is_increasing = Some(level_int < next_level_int);
    }
    return is_safe

}

fn part_one() {
    let mut num_safe_reports = 0;

    let input = fs::read_to_string("Day 02/input.txt").expect("");
    for line in input.lines() {
        let levels: Vec<&str>= line.split(" ").collect();
        if line_is_safe(&levels)
 {
            num_safe_reports += 1;
        } 
    }
    println!("{}", num_safe_reports);
}

fn part_two() {
    let mut num_safe_reports = 0;

    let input = fs::read_to_string("Day 02/input.txt").expect("");
    for line in input.lines() {
        let levels: Vec<&str> = line.split(" ").collect();
        if line_is_safe(&levels) {
            num_safe_reports += 1
        } else {
            for idx in 0..levels.len() {
                let mut tmp_levels = levels.clone();
                tmp_levels.remove(idx);

                if line_is_safe(&tmp_levels) {
                    num_safe_reports += 1;
                    break
                }
        }
        } 
    }
    println!("{}", num_safe_reports);
}

fn main() {
    part_one();
    part_two();
}