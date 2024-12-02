advent_of_code::solution!(1);

fn input_to_cols(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut col_r: Vec<i32> = Vec::new();
    let mut col_l: Vec<i32> = Vec::new();
    for line in input.lines() {
        let both_col: Vec<&str> = line.split_whitespace().collect();

        match (both_col[0].parse(), both_col[1].parse()) {
            (Ok(left), Ok(right)) => {
                col_l.push(left);
                col_r.push(right);
            }
            _ => eprintln!("Failed to parse line: {}", line),
        }
    }

    (col_l, col_r)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut col_l, mut col_r) = input_to_cols(input);
    col_l.sort();
    col_r.sort();

    let total_dist = col_l
        .iter()
        .zip(col_r.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    Some(total_dist as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (col_l, col_r) = input_to_cols(input);

    // More efficient and idiomatic approach
    let tot_similarity = col_l
        .iter()
        .map(|&sample| {
            let sim = col_r.iter().filter(|&&x| x == sample).count() as i32;
            sample * sim
        })
        .sum::<i32>();

    Some(tot_similarity as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("Part 1: {:?}", result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("Part 2: {:?}", result);
    }
}
