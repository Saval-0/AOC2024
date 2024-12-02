use itertools::Itertools;

advent_of_code::solution!(2);

fn get_level_vec(input: &str) -> Vec<Vec<i32>> {
    let levels = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .flat_map(|el| el.parse().ok())
                .collect_vec()
        })
        .collect();

    return levels;
}

fn is_safe_rem(level: &Vec<i32>) -> bool {
    if !is_safe(level) {
        for i in 0..level.len() {
            let mut temp = level.clone();
            temp.remove(i);
            if is_safe(&temp) {
                return true;
            }
        }
        return false;
    }

    return true;
}

fn is_safe(level: &Vec<i32>) -> bool {
    return is_asc_or_desc(level) && are_incs_safe(level);
}

fn is_asc_or_desc(level: &Vec<i32>) -> bool {
    let asc = level.windows(2).all(|v| v[0] <= v[1]);
    let desc = level.windows(2).all(|v| v[0] >= v[1]);
    return asc ^ desc;
}

fn are_incs_safe(level: &Vec<i32>) -> bool {
    let incs = level
        .iter()
        .tuple_windows()
        .map(|(a, b)| (a - b).abs())
        .collect_vec();

    return incs.iter().all(|&inc| (1..=3).contains(&inc));
}

pub fn part_one(input: &str) -> Option<u32> {
    let levels = get_level_vec(input);

    let mut count = 0;
    for level in levels {
        if is_safe(&level) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let levels = get_level_vec(input);

    let mut count = 0;
    for level in levels {
        if is_safe_rem(&level) {
            count += 1;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("Part 1: {:?}", result);
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("Part 2: {:?}", result);
        // assert_eq!(result, None);
    }
}
