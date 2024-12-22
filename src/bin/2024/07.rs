advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;

    for line in input.lines() {
        let sides: Vec<&str> = line.split(":").collect();
        let desired = sides[0].parse::<u64>().unwrap();
        let mut nums: Vec<u64> = sides[1]
            .split_whitespace()
            .map(|string| string.parse::<u64>().unwrap())
            .collect();
        nums.reverse();

        if solve_pattern(desired, &nums, false) {
            total += desired;
        }
    }

    return Some(total);
}

fn solve_pattern(desired: u64, nums: &[u64], is_pt2: bool) -> bool {
    if nums.len() == 0 {
        return desired == 0;
    }

    if desired == 0 {
        return true;
    }

    let mut ret_val = false;
    let next_num = nums[0];
    if desired % next_num == 0 {
        let mut diff = 0;
        if desired == next_num {
            diff = 1;
        }
        ret_val = ret_val || solve_pattern((desired / next_num) - diff, &nums[1..], is_pt2)
    }
    if desired >= next_num {
        ret_val = ret_val || solve_pattern(desired - next_num, &nums[1..], is_pt2);
    }

    if is_pt2 && desired.to_string().ends_with(&next_num.to_string()) {
        let desired_str = desired.to_string();
        let num_str = next_num.to_string();
        if let Some(stripped) = desired_str.strip_suffix(&num_str) {
            let new_desired = if stripped.is_empty() {
                0
            } else {
                stripped.parse::<u64>().unwrap()
            };
            ret_val = ret_val || solve_pattern(new_desired, &nums[1..], is_pt2);
        }
    }
    return ret_val;
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;

    for line in input.lines() {
        let sides: Vec<&str> = line.split(":").collect();
        let desired = sides[0].parse::<u64>().unwrap();
        let mut nums: Vec<u64> = sides[1]
            .split_whitespace()
            .map(|string| string.parse::<u64>().unwrap())
            .collect();
        nums.reverse();

        if solve_pattern(desired, &nums, true) {
            total += desired;
        }
    }

    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
