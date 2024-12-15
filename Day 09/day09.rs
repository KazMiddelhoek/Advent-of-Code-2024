use std::fs;

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

fn part_two() {}

fn main() {
    part_one();
    part_two();
}
