use std::fs;
fn look_around_for_M(array: &Vec<Vec<char>>, current_pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut all_posibilities = Vec::new();
    for y in -1isize..2 {
        for x in -1isize..2 {
            if current_pos.0 == 0 && y == -1 {
                continue;
            }
            if current_pos.0 == (array.len()-1) as isize && y == 1 {
                continue;
            }
            if current_pos.1 == 0 && x == -1 {
                continue;
            }
            if current_pos.1 == (array[0].len()-1) as isize && x == 1 {
                continue;
            }
            if array[(current_pos.0 + y) as usize][(current_pos.1 + x) as usize] == 'M' {
                all_posibilities.push((y, x))
            }
        }
    }
    return all_posibilities
}

fn part_one() {
    let word_search = fs::read_to_string("Day 04/input.txt").expect("");
    let mut word_search_array = Vec::new();
    for line in word_search.lines() {
        word_search_array.push(line.chars().collect::<Vec<char>>());
   }

   let mut n_xmas = 0;
   for (line_idx, line) in word_search_array.iter().enumerate() {
    for (letter_idx, letter) in line.iter().enumerate() {
        if *letter == 'X' {
            let options = look_around_for_M(&word_search_array, (line_idx as isize, letter_idx as isize));
            for option in options {
               if line_idx as isize + (2 * option.0) < 0 ||  line_idx as isize + (3 * option.0) < 0 {
                continue;
               }

               if line_idx as isize + (2 * option.0) >= word_search_array.len() as isize ||  line_idx as isize + (3 * option.0) >= word_search_array.len() as isize { 
                continue;
               }
               if letter_idx as isize + (2 * option.1) < 0 ||  letter_idx as isize + (3 * option.1) < 0 {
                continue;
               }

               if letter_idx as isize + (2 * option.1)  >= word_search_array.len() as isize ||  letter_idx as isize + (3 * option.1) >= word_search_array.len() as isize {
                continue;
               }
                if word_search_array[(line_idx as isize + (2 * option.0)) as usize][(letter_idx as isize + (2 * option.1)) as usize] == 'A' {
                    if word_search_array[(line_idx as isize + (3 * option.0)) as usize][(letter_idx as isize+ (3 * option.1)) as usize] == 'S' {
                        n_xmas +=1
                    }
                }
            }
        }
    }
   }

   println!("{}", n_xmas)
}

fn part_two() {
    let word_search = fs::read_to_string("Day 04/input.txt").expect("");
    let mut word_search_array = Vec::new();
    for line in word_search.lines() {
        word_search_array.push(line.chars().collect::<Vec<char>>());
   }

   let mut n_xmas = 0;
   for (line_idx, line) in word_search_array.iter().enumerate() {
        if line_idx == 0 || line_idx == word_search_array.len() -1 {
            continue;
        }
    for (letter_idx, letter) in line.iter().enumerate() {
        if letter_idx == 0 || letter_idx == word_search_array[0].len() -1 {
            continue;
        }
        if *letter == 'A' {
            if word_search_array[line_idx-1][letter_idx-1] == 'M' &&  
                word_search_array[line_idx+1][letter_idx-1] == 'M' &&
                word_search_array[line_idx+1][letter_idx+1] == 'S' &&
                word_search_array[line_idx-1][letter_idx+1] == 'S'
             { n_xmas +=1}

            if word_search_array[line_idx-1][letter_idx-1] == 'S' &&  
                word_search_array[line_idx+1][letter_idx-1] == 'S' &&
                word_search_array[line_idx+1][letter_idx+1] == 'M' &&
                word_search_array[line_idx-1][letter_idx+1] == 'M'
             { n_xmas +=1}
            if word_search_array[line_idx-1][letter_idx-1] == 'S' &&  
                word_search_array[line_idx+1][letter_idx-1] == 'M' &&
                word_search_array[line_idx+1][letter_idx+1] == 'M' &&
                word_search_array[line_idx-1][letter_idx+1] == 'S'
             { n_xmas +=1}
            if word_search_array[line_idx-1][letter_idx-1] == 'M' &&  
                word_search_array[line_idx+1][letter_idx-1] == 'S' &&
                word_search_array[line_idx+1][letter_idx+1] == 'S' &&
                word_search_array[line_idx-1][letter_idx+1] == 'M'
             { n_xmas +=1}
        }
    }
   }
   println!("{}", n_xmas)
}

fn main() {
    part_one();
    part_two();
}