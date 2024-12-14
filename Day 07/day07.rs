use std::fs;

fn recurse(numbers: Vec<i64>, current_sum: i64, output: i64) -> bool {
    if numbers.len() == 0 && current_sum == output {
        return true;
    }
    if numbers.len() == 0 && current_sum != output {
        return false;
    }
    if current_sum > output {return false}

    let success = recurse(numbers[1..].to_vec(), current_sum+numbers[0], output);
    if success {return success}

    if current_sum == 0 {
        let success = recurse(numbers[1..].to_vec(), numbers[0], output);
        if success {return success}
    } else {
        let success = recurse(numbers[1..].to_vec(), current_sum*numbers[0], output);
        if success {return success}
    }

    // part two
    let mut sum_string = current_sum.to_string();
    sum_string.push_str(&numbers[0].to_string());
    let success = recurse(numbers[1..].to_vec(), sum_string.parse::<i64>().expect(""), output);
    if success {return success}

    return false;
}

fn part_one_and_two() {
    let input = fs::read_to_string("Day 07/input.txt").expect("");
    let mut total_calibration_result: i64 = 0;
    for line in input.lines() {
        let mut iterator = line.splitn(2, ": ");
         
        let output = iterator.next().expect("").parse::<i64>().expect("");
        let numbers = iterator.next().expect("").split(" ").map(|x|  x.parse::<i64>().expect("")).collect::<Vec<i64>>();

        let success = recurse(numbers, 0, output);        
        if success {
            total_calibration_result += output;
        }
    }
    println!("{}", total_calibration_result)

}


fn main() {
part_one_and_two();
}