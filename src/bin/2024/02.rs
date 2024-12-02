use std::thread::current;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut valid_lines: u32 = 0;
    for line in input.lines() {
        // println!("{}", line);
        let items: Vec<&str> = line.split(' ').collect();
        let is_valid = check_valid_report(items);
        if is_valid {
            valid_lines = valid_lines + 1;
        }
    }
    return Some(valid_lines);
}

fn check_valid_report(items: Vec<&str>) -> bool {
    let mut curr_item: i32 = items[0].parse().expect("Failed");
    let mut next_item: i32 = items[1].parse().expect("Failed");
    let is_decreasing = curr_item > next_item;

    for i in 0..(items.len() - 1) {
        curr_item = items[i].parse().expect("Failed");
        next_item = items[i + 1].parse().expect("Failed");
        let diff: i32;
        if is_decreasing {
            diff = curr_item - next_item;
        } else {
            diff = next_item - curr_item;
        }
        if !(diff <= 3 && diff > 0) {
            return false;
        }
    }
    return true;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut valid_lines: u32 = 0;
    for line in input.lines() {
        let items: Vec<&str> = line.split(' ').collect();
        let is_valid = check_valid_report(items.clone());
        if is_valid {
            valid_lines = valid_lines + 1;
        } else {
            for idx in 0..items.len() {
                let removed_items = [&items[0..idx], &items[idx + 1..]].concat();
                let check_part_2 = check_valid_report(removed_items);
                if check_part_2 {
                    valid_lines += 1;
                    break;
                }
            }
        }
    }
    return Some(valid_lines);
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
