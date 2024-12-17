use std::{collections::HashSet, fs};

fn part_one() {
    let input = fs::read_to_string("Day 10/input.txt")
        .expect("")
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).expect(""))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut sum_of_scores = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, n) in row.iter().enumerate() {
            if *n != 0 {
                continue;
            }
            let reachable_9s: HashSet<(usize, usize)> = HashSet::new();
            sum_of_scores += move_over(&input, (i, j), reachable_9s).len();
        }
    }
    println!("{}", sum_of_scores)
}

fn move_over(
    map: &Vec<Vec<u32>>,
    current_pos: (usize, usize),
    mut reachable_9s: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    if map[current_pos.0][current_pos.1] == 9 {
        reachable_9s.insert(current_pos);
        return reachable_9s;
    }

    // move up
    if current_pos.0 != 0 {
        if map[current_pos.0 - 1][current_pos.1] > map[current_pos.0][current_pos.1]
            && map[current_pos.0 - 1][current_pos.1] - map[current_pos.0][current_pos.1] == 1
        {
            reachable_9s = move_over(map, (current_pos.0 - 1, current_pos.1), reachable_9s);
        }
    }
    // move down
    if current_pos.0 != map.len() - 1 {
        if map[current_pos.0 + 1][current_pos.1] > map[current_pos.0][current_pos.1]
            && map[current_pos.0 + 1][current_pos.1] - map[current_pos.0][current_pos.1] == 1
        {
            reachable_9s = move_over(map, (current_pos.0 + 1, current_pos.1), reachable_9s);
        }
    }

    // move left
    if current_pos.1 != 0 {
        if map[current_pos.0][current_pos.1 - 1] > map[current_pos.0][current_pos.1]
            && map[current_pos.0][current_pos.1 - 1] - map[current_pos.0][current_pos.1] == 1
        {
            reachable_9s = move_over(map, (current_pos.0, current_pos.1 - 1), reachable_9s);
        }
    }
    // move right
    if current_pos.1 != map.len() - 1 {
        if map[current_pos.0][current_pos.1 + 1] > map[current_pos.0][current_pos.1]
            && map[current_pos.0][current_pos.1 + 1] - map[current_pos.0][current_pos.1] == 1
        {
            reachable_9s = move_over(map, (current_pos.0, current_pos.1 + 1), reachable_9s);
        }
    }
    return reachable_9s;
}

fn part_two() {
    let input = fs::read_to_string("Day 10/input.txt")
        .expect("")
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).expect(""))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut sum_of_scores = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, n) in row.iter().enumerate() {
            if *n != 0 {
                continue;
            }
            sum_of_scores += move_over_rating(&input, (i, j));
        }
    }
    println!("{}", sum_of_scores)
}

fn move_over_rating(map: &Vec<Vec<u32>>, current_pos: (usize, usize)) -> i32 {
    if map[current_pos.0][current_pos.1] == 9 {
        return 1;
    }

    let mut sum_of_scores = 0;

    // move up
    if current_pos.0 != 0 {
        if map[current_pos.0 - 1][current_pos.1] > map[current_pos.0][current_pos.1]
            && map[current_pos.0 - 1][current_pos.1] - map[current_pos.0][current_pos.1] == 1
        {
            sum_of_scores += move_over_rating(map, (current_pos.0 - 1, current_pos.1));
        }
    }
    // move down
    if current_pos.0 != map.len() - 1 {
        if map[current_pos.0 + 1][current_pos.1] > map[current_pos.0][current_pos.1]
            && map[current_pos.0 + 1][current_pos.1] - map[current_pos.0][current_pos.1] == 1
        {
            sum_of_scores += move_over_rating(map, (current_pos.0 + 1, current_pos.1));
        }
    }

    // move left
    if current_pos.1 != 0 {
        if map[current_pos.0][current_pos.1 - 1] > map[current_pos.0][current_pos.1]
            && map[current_pos.0][current_pos.1 - 1] - map[current_pos.0][current_pos.1] == 1
        {
            sum_of_scores += move_over_rating(map, (current_pos.0, current_pos.1 - 1));
        }
    }
    // move right
    if current_pos.1 != map.len() - 1 {
        if map[current_pos.0][current_pos.1 + 1] > map[current_pos.0][current_pos.1]
            && map[current_pos.0][current_pos.1 + 1] - map[current_pos.0][current_pos.1] == 1
        {
            sum_of_scores += move_over_rating(map, (current_pos.0, current_pos.1 + 1));
        }
    }
    return sum_of_scores;
}

fn main() {
    part_one();
    part_two();
}
