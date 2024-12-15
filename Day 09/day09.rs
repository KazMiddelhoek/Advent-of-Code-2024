use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part_one() {
    let mut input = fs::read_to_string("Day 09/input.txt")
        .expect("msg")
        .lines()
        .next()
        .expect("msg")
        .chars()
        .map(|x| x.to_digit(10).expect(""))
        .collect::<Vec<u32>>();
    let mut end_digit_idx = input.len() - 1;
    let mut free_space_idx = 1;
    let mut checksum = 0;
    let mut current_block_pos = input[0];

    loop {
        if end_digit_idx <= free_space_idx {
            break;
        }

        let n_to_remove = (input[end_digit_idx]).min(input[free_space_idx]);
        input[free_space_idx] -= n_to_remove;

        checksum += (current_block_pos..current_block_pos + n_to_remove)
            .map(|x| x * end_digit_idx as u32 / 2)
            .sum::<u32>() as i64;
        current_block_pos += n_to_remove;

        input[end_digit_idx] -= n_to_remove;

        if input[free_space_idx] == 0 {
            checksum += (current_block_pos..current_block_pos + input[free_space_idx + 1])
                .map(|x| x * (free_space_idx as u32 + 1) / 2)
                .sum::<u32>() as i64;
            current_block_pos += input[free_space_idx + 1];
            free_space_idx += 2
        }
        if input[end_digit_idx] == 0 {
            end_digit_idx -= 2
        }
    }
    println!("{:?}", checksum);
}

fn part_two() {
    let mut input = fs::read_to_string("Day 09/input.txt")
        .expect("msg")
        .lines()
        .next()
        .expect("msg")
        .chars()
        .map(|x| x.to_digit(10).expect(""))
        .collect::<Vec<u32>>();
    let mut end_digit_idx = input.len() - 1;
    let mut free_space_map: HashMap<usize, Vec<(usize, u32)>> = HashMap::new();
    let mut moved_set = HashSet::new();

    loop {
        if end_digit_idx == 0 {
            break;
        }

        let n_to_remove = input[end_digit_idx];

        // input[free_space_idx] -= n_to_remove;
        let mut can_move = true;
        let mut free_space_idx = 1;
        while (!free_space_map.contains_key(&free_space_idx) && input[free_space_idx] < n_to_remove)
            || (free_space_map.contains_key(&free_space_idx)
                && input[free_space_idx]
                    - free_space_map
                        .get(&free_space_idx)
                        .expect("")
                        .iter()
                        .map(|x| x.1)
                        .sum::<u32>()
                    < n_to_remove)
        {
            free_space_idx += 2;
            if free_space_idx >= end_digit_idx {
                can_move = false;
                break;
            }
        }
        if can_move {
            free_space_map
                .entry(free_space_idx)
                .and_modify(|x| x.push((end_digit_idx / 2, input[end_digit_idx])))
                .or_insert(vec![(end_digit_idx / 2, input[end_digit_idx])]);
            moved_set.insert(end_digit_idx / 2);
        }

        end_digit_idx -= 2;
    }

    // calculate checksum
    let mut checksum = 0;
    let mut current_block_pos = 0;
    let mut current_idx = 0;

    // First checksum for unmoved files
    while current_idx < input.len() {
        if !moved_set.contains(&(current_idx / 2)) {
            checksum += (current_block_pos..current_block_pos + input[current_idx])
                .map(|x| x * current_idx as u32 / 2)
                .sum::<u32>() as i64;
        }
        if current_idx == input.len() - 1 {
            break;
        }
        current_block_pos += input[current_idx] + input[current_idx + 1];
        current_idx += 2;
    }

    // then for the moved files.
    current_block_pos = input[0];
    current_idx = 1;

    while current_idx < input.len() {
        if free_space_map.contains_key(&current_idx) {
            let vecs = free_space_map.get(&current_idx).expect("");
            let mut temp_current_block_pos = current_block_pos;
            for tup in vecs {
                checksum += (temp_current_block_pos..temp_current_block_pos + tup.1)
                    .map(|x| x * tup.0 as u32)
                    .sum::<u32>() as i64;
                temp_current_block_pos += tup.1;
            }
        }
        if current_idx == input.len() - 1 {
            break;
        }
        current_block_pos += input[current_idx] + input[current_idx + 1];
        current_idx += 2;
    }

    println!("{:?}", checksum);
}

fn main() {
    part_one();
    part_two();
}
