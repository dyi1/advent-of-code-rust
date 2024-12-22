advent_of_code::solution!(19);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let towels: Vec<String> = sections[0].split(", ").map(|t| t.to_string()).collect();
    let patterns: Vec<&str> = sections[1].lines().collect();
    let mut cache: HashMap<String, u64> = HashMap::new();
    let total: u64 = patterns
        .iter()
        .map(|pattern| {
            if solve_pattern(pattern, &towels, &mut cache) > 0 {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    return Some(total);
}

fn solve_pattern(pattern: &str, towels: &[String], cache: &mut HashMap<String, u64>) -> u64 {
    // Check cache first
    if let Some(&result) = cache.get(pattern) {
        return result;
    }

    // Base case
    if pattern.is_empty() {
        return 1;
    }

    // Try each towel pattern
    let result = towels
        .iter()
        .filter(|t| pattern.starts_with(*t))
        .map(|t| solve_pattern(&pattern[t.len()..], towels, cache))
        .sum();

    // Store result in cache before returning
    cache.insert(pattern.to_string(), result);
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let towels: Vec<String> = sections[0].split(", ").map(|t| t.to_string()).collect();
    let patterns: Vec<&str> = sections[1].lines().collect();
    let mut cache: HashMap<String, u64> = HashMap::new();
    let total: u64 = patterns
        .iter()
        .map(|pattern| solve_pattern(pattern, &towels, &mut cache))
        .sum();

    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
