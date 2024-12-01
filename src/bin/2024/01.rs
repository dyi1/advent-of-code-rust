advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        let left_num = left.parse::<i32>().unwrap();
        let right_num = right.parse::<i32>().unwrap();

        left_vec.push(left_num);
        right_vec.push(right_num);
    }

    left_vec.sort();
    right_vec.sort();
    let mut total_diff = 0;
    for i in 0..left_vec.len() {
        total_diff += right_vec[i].abs_diff(left_vec[i]);
    }
    return Some(total_diff);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_vec: Vec<u32> = Vec::new();
    let mut right_vec: Vec<u32> = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        let left_num = left.parse::<u32>().unwrap();
        let right_num = right.parse::<u32>().unwrap();

        left_vec.push(left_num);
        right_vec.push(right_num);
    }

    left_vec.sort();
    right_vec.sort();

    let mut count: u32 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut score: u32 = 0;
    let mut current_right: u32 = right_vec[0];
    // let mut current_left = left_vec[0];
    // let mut current_right = right_vec[0];
    while i < left_vec.len() && j < right_vec.len() {
        if right_vec[j] == current_right {
            count += 1;
            j += 1;
            continue;
        }
        if current_right == left_vec[i] {
            score += left_vec[i] * count;
            i += 1;
            continue;
        }
        if current_right > left_vec[i] {
            i += 1;
            continue;
        }
        if left_vec[i] > current_right {
            current_right = right_vec[j];
            count = 0;
            continue;
        }
    }
    return Some(score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
