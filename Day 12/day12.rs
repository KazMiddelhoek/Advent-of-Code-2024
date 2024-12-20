use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

fn part_one() {
    let garden = fs::read_to_string("Day 12/input.txt")
        .expect("")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut regions_per_plant = HashMap::<char, Vec<HashSet<(usize, usize)>>>::new();

    for (i, line) in garden.iter().enumerate() {
        for (j, plant) in line.iter().enumerate() {
            if regions_per_plant.contains_key(plant)
                && regions_per_plant
                    .get(plant)
                    .expect("")
                    .iter()
                    .any(|x| x.contains(&(i, j)))
            {
                continue;
            }
            let mut new_region = HashSet::new();
            new_region = look_around(&garden, (i, j), new_region);

            regions_per_plant
                .entry(*plant)
                .and_modify(|x| x.push(new_region.clone()))
                .or_insert(vec![new_region.clone()]);
        }
    }

    // Now calculate prices
    let mut total_price = 0;
    for (_plant, regions) in regions_per_plant {
        for region in regions {
            let mut circum = 0;
            for pos in &region {
                if pos.0 == garden.len() - 1 || !region.contains(&(pos.0 + 1, pos.1)) {
                    circum += 1
                }
                if pos.0 == 0 || !region.contains(&(pos.0 - 1, pos.1)) {
                    circum += 1
                }
                if pos.1 == garden[0].len() - 1 || !region.contains(&(pos.0, pos.1 + 1)) {
                    circum += 1
                }
                if pos.1 == 0 || !region.contains(&(pos.0, pos.1 - 1)) {
                    circum += 1
                }
            }
            total_price += region.len() * circum;
        }
    }
    println!("{:?}", total_price)
}

fn look_around(
    garden: &Vec<Vec<char>>,
    pos: (usize, usize),
    mut new_region: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    new_region.insert(pos);
    // move up
    if pos.0 != 0 {
        if garden[pos.0 - 1][pos.1] == garden[pos.0][pos.1]
            && !new_region.contains(&(pos.0 - 1, pos.1))
        {
            new_region = look_around(garden, (pos.0 - 1, pos.1), new_region);
        }
    }
    // move down
    if pos.0 != garden.len() - 1 {
        if garden[pos.0 + 1][pos.1] == garden[pos.0][pos.1]
            && !new_region.contains(&(pos.0 + 1, pos.1))
        {
            new_region = look_around(garden, (pos.0 + 1, pos.1), new_region);
        }
    }

    // move left
    if pos.1 != 0 {
        if garden[pos.0][pos.1 - 1] == garden[pos.0][pos.1]
            && !new_region.contains(&(pos.0, pos.1 - 1))
        {
            new_region = look_around(garden, (pos.0, pos.1 - 1), new_region);
        }
    }
    // move right
    if pos.1 != garden[0].len() - 1 {
        if garden[pos.0][pos.1 + 1] == garden[pos.0][pos.1]
            && !new_region.contains(&(pos.0, pos.1 + 1))
        {
            new_region = look_around(garden, (pos.0, pos.1 + 1), new_region);
        }
    }

    return new_region;
}

fn part_two() {
    let garden = fs::read_to_string("Day 12/input.txt")
        .expect("")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut regions_per_plant = HashMap::<char, Vec<HashSet<(usize, usize)>>>::new();

    for (i, line) in garden.iter().enumerate() {
        for (j, plant) in line.iter().enumerate() {
            if regions_per_plant.contains_key(plant)
                && regions_per_plant
                    .get(plant)
                    .expect("")
                    .iter()
                    .any(|x| x.contains(&(i, j)))
            {
                continue;
            }
            let mut new_region = HashSet::new();
            new_region = look_around(&garden, (i, j), new_region);

            regions_per_plant
                .entry(*plant)
                .and_modify(|x| x.push(new_region.clone()))
                .or_insert(vec![new_region.clone()]);
        }
    }

    let mut total_price = 0;
    for (_plant, regions) in regions_per_plant {
        for region in regions {
            let mut circum = 0;
            for pos in &region {
                // look down
                if (pos.0 == garden.len() - 1 || !region.contains(&(pos.0 + 1, pos.1)))
                    && (pos.1 == 0
                        || !region.contains(&(pos.0, pos.1 - 1))
                        || (region.contains(&(pos.0, pos.1 - 1))
                            && region.contains(&(pos.0 + 1, pos.1 - 1))))
                {
                    circum += 1
                }
                // look up
                if (pos.0 == 0 || !region.contains(&(pos.0 - 1, pos.1)))
                    && (pos.1 == 0
                        || !region.contains(&(pos.0, pos.1 - 1))
                        || (region.contains(&(pos.0, pos.1 - 1)))
                            && (pos.0 != 0 && region.contains(&(pos.0 - 1, pos.1 - 1))))
                {
                    circum += 1
                }
                // look right
                if (pos.1 == garden[0].len() - 1 || !region.contains(&(pos.0, pos.1 + 1)))
                    && (pos.0 == 0
                        || !region.contains(&(pos.0 - 1, pos.1))
                        || (region.contains(&(pos.0 - 1, pos.1)))
                            && region.contains(&(pos.0 - 1, pos.1 + 1)))
                {
                    circum += 1
                }
                // look left
                if (pos.1 == 0 || !region.contains(&(pos.0, pos.1 - 1)))
                    && (pos.0 == 0 || // im at top
                        !region.contains(&(pos.0 - 1, pos.1)) // above is not in my region
                        || (region.contains(&(pos.0 - 1, pos.1)) // above is in my region, but upper left is too.
                            && (pos.1!=0 && region.contains(&(pos.0 - 1, pos.1 - 1)))))
                {
                    circum += 1
                }
            }
            total_price += region.len() * circum;
        }
    }
    println!("{:?}", total_price)
}

fn main() {
    part_one();
    part_two();
}
