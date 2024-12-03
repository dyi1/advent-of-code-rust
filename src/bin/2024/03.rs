use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    let capture_matches: regex::CaptureMatches<'_, '_> = re.captures_iter(input);
    let mut total = 0;
    for (_capture, [a, b]) in capture_matches.map(|cap| cap.extract()) {
        // println!("{}", capture);
        let num1 = a.parse::<u32>().unwrap();
        let num2 = b.parse::<u32>().unwrap();
        total += num1 * num2;
    }
    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(don't\(\)|do\(\))").unwrap();
    let parts: Vec<&str> = re.split(input).collect();
    let delimiters: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    let mut total: u32 = 0;

    // First is enabled by default
    let first_val = parts[0];
    total += part_one(first_val).unwrap();

    // Start at the second value
    let mut parts_iter = parts.iter();
    parts_iter.next();
    for (delimiter, text) in delimiters.iter().zip(parts_iter) {
        if delimiter.eq(&"do()") {
            // println!("{}, {}", delimiter, text);
            total += part_one(text).unwrap();
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
