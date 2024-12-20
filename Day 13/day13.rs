use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

fn part_one() {
    let mut sum_of_tokens = 0.0;
    let binding = fs::read_to_string("Day 13/input.txt").expect("");
    let mut lines = binding.lines();
    while let Some(line) = lines.next() {
        let [a, c] = line
            .strip_prefix("Button A: X+")
            .expect("")
            .split(", Y+")
            .map(|x| x.parse::<f64>().expect(""))
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let [b, d] = lines
            .next()
            .expect("")
            .strip_prefix("Button B: X+")
            .expect("")
            .split(", Y+")
            .map(|x| x.parse::<f64>().expect(""))
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let [prize_x, prize_y] = lines
            .next()
            .expect("")
            .strip_prefix("Prize: X=")
            .expect("")
            .split(", Y=")
            .map(|x| x.parse::<f64>().expect(""))
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        lines.next(); // blank line

        let det = a * d - b * c;
        if det == 0.0 {
            continue;
        }

        let ans_A = (d * prize_x + -b * prize_y) / det;
        let ans_B = (-c * prize_x + a * prize_y) / det;
        if ans_A.floor() != ans_A || ans_B.floor() != ans_B {
            continue;
        }
        if ans_A > 100.0 || ans_B > 100.0 {
            continue;
        }
        println!("{} {}", ans_A, ans_B);
        sum_of_tokens += ans_A * 3.0 + ans_B;

        // [A,B] = A-1 * prize
        // [94, 22 [A  = [8400
        //  34, 67]  B]    5400]

        // A-1 = 1/(94*67 - 22*34)[67 -22
        // -34 94]
    }
    println!("{}", sum_of_tokens)
}

fn part_two() {
    let mut sum_of_tokens = 0.0;
    let binding = fs::read_to_string("Day 13/input.txt").expect("");
    let mut lines = binding.lines();
    while let Some(line) = lines.next() {
        let [a, c] = line
            .strip_prefix("Button A: X+")
            .expect("")
            .split(", Y+")
            .map(|x| x.parse::<f64>().expect(""))
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let [b, d] = lines
            .next()
            .expect("")
            .strip_prefix("Button B: X+")
            .expect("")
            .split(", Y+")
            .map(|x| x.parse::<f64>().expect(""))
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let [mut prize_x, mut prize_y] = lines
            .next()
            .expect("")
            .strip_prefix("Prize: X=")
            .expect("")
            .split(", Y=")
            .map(|x| x.parse::<f64>().expect(""))
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        lines.next(); // blank line

        prize_x += 10000000000000.0;
        prize_y += 10000000000000.0;

        let det = a * d - b * c;
        if det == 0.0 {
            continue;
        }

        let ans_A = (d * prize_x + -b * prize_y) / det;
        let ans_B = (-c * prize_x + a * prize_y) / det;
        if ans_A.floor() != ans_A || ans_B.floor() != ans_B {
            continue;
        }
        println!("{} {}", ans_A, ans_B);
        sum_of_tokens += ans_A * 3.0 + ans_B;
    }
    println!("{}", sum_of_tokens)
}

fn main() {
    part_one();
    part_two();
}
