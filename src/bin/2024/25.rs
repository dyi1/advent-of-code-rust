advent_of_code::solution!(25);

#[derive(Debug)]
struct Entry {
    entry_type: String,
    values: (u32, u32, u32, u32, u32),
}

pub fn part_one(input: &str) -> Option<u32> {
    let entries = input.split("\n\n").map(|entry| {
        let result: Vec<Vec<char>> = entry.lines().map(|line| line.chars().collect()).collect();
        let is_all_hash = result[0].iter().all(|&c| c == '#');
        let result_type = match is_all_hash {
            true => "lock",
            false => "key",
        };
        let mut pins: Vec<u32> = vec![];
        for j in 0..result[0].len() {
            let mut num = 0;
            for i in 1..result.len() - 1 {
                if result[i][j] == '#' {
                    num += 1;
                }
            }
            pins.push(num);
        }
        return Entry {
            entry_type: result_type.to_string(),
            values: (pins[0], pins[1], pins[2], pins[3], pins[4]),
        };
    });
    let mut locks = vec![];
    let mut keys = vec![];
    for e in entries {
        // println!("{:?}", e);
        if e.entry_type == "lock" {
            locks.push(e);
        } else {
            keys.push(e);
        }
    }

    let mut total = 0;
    for k in &keys {
        for l in &locks {
            let (k1, k2, k3, k4, k5) = k.values;
            let (l1, l2, l3, l4, l5) = l.values;
            let zipped = [(k1, l1), (k2, l2), (k3, l3), (k4, l4), (k5, l5)];
            let fits = zipped.iter().all(|(k, l)| k + l <= 5);
            if fits {
                total += 1;
            }
        }
    }

    return Some(total);
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
