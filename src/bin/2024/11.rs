use std::collections::{btree_map::Values, HashMap};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_stones: Vec<String> = Vec::new();
    for stone in input.split_whitespace() {
        current_stones.push(stone.to_string());
    }
    println!("Length of input {}", current_stones.len());
    for _ in 0..25 {
        let mut next_stones: Vec<String> = Vec::new();
        for stone in &current_stones {
            if cache.contains_key(stone) {
                if let Some(cached_stones) = cache.get(stone) {
                    next_stones.extend(cached_stones.clone());
                }
            } else {
                let next_stone = convert_next_stone(stone);
                cache.insert(stone.clone(), next_stone.clone());
                next_stones.extend(next_stone);
            }
        }
        current_stones = next_stones;
        // println!("{:?}", current_stones);
    }
    // println!("{}", input);
    return Some(current_stones.len() as u32);
}

fn convert_next_stone(stone: &str) -> Vec<String> {
    if stone == "0" {
        return vec!["1".to_string()];
    }
    if stone.len() % 2 == 0 {
        let (left, right) = split_and_trim_zeros(stone);
        return vec![left.to_string(), right.to_string()];
    }

    let num = stone.parse::<u64>().unwrap_or(0);
    let new_val = num * 2024;
    return vec![new_val.to_string()];
}

fn split_and_trim_zeros(stone: &str) -> (&str, &str) {
    // Find the middle point
    let mid = stone.len() / 2;

    // Split into left and right parts
    let left = &stone[..mid];
    let right = &stone[mid..];

    // Trim leading zeros from right part
    let trimmed_right = right.trim_start_matches('0');

    // If trimmed_right is empty (all zeros), return "0"
    let final_right = if trimmed_right.is_empty() {
        "0"
    } else {
        trimmed_right
    };

    (left, final_right)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stone_cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut count_cache: HashMap<(u32, String), u32> = HashMap::new();

    let mut total: u32 = 0;
    for stone in input.split_whitespace() {
        total += count_stones(stone.to_string(), 25, &mut stone_cache, &mut count_cache);
        println!("{}", total);
    }
    // println!("{}", input);
    return Some(total);
}

fn count_stones(
    stone: String,
    layer: u32,
    stone_cache: &mut HashMap<String, Vec<String>>,
    count_cache: &mut HashMap<(u32, String), u32>,
) -> u32 {
    if layer == 0 {
        return 1;
    }

    let count_cache_key = &(layer, stone.clone());
    if count_cache.contains_key(count_cache_key) {
        if let Some(counter) = count_cache.get(count_cache_key) {
            return *counter;
        }
    }
    let mut counter: u32 = 0;
    let next_layer: Vec<String>;
    if let Some(values) = stone_cache.get(&stone) {
        next_layer = values.to_vec()
    } else {
        next_layer = convert_next_stone(&stone);
        stone_cache.insert(stone.clone(), next_layer.clone());
    };

    for n in next_layer {
        counter += count_stones(n, layer - 1, stone_cache, count_cache)
    }
    // Update the cache
    count_cache.insert((layer, stone), counter);
    return counter;
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
