use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut invalid_rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut total: u32 = 0;
    for line in input.lines() {
        // Construct rules
        let rule: Vec<&str> = line.split("|").collect();
        if rule.len() > 1 {
            let prev: &str = rule[0];
            let next: &str = rule[1];
            if !invalid_rules.contains_key(prev) {
                invalid_rules.insert(prev, vec![]);
            }
            let invalid_prev = invalid_rules.get_mut(prev).unwrap();
            invalid_prev.push(next);
            continue;
        }
        // println!("{:?}", invalid_rules);
        // start processing items
        let list_items: Vec<&str> = line.split(",").collect();
        if list_items.len() > 1 {
            if is_valid(&invalid_rules, &list_items) {
                // println!("{:?}", list_items);
                let center_item = list_items[list_items.len() / 2].parse::<u32>().unwrap();
                // println!("{}", center_item);
                total += center_item;
            }
        }
    }
    return Some(total);
}

fn is_valid(invalid_rules: &HashMap<&str, Vec<&str>>, list_items: &Vec<&str>) -> bool {
    let mut seen_items: HashSet<&str> = HashSet::new();
    for item in list_items {
        if !invalid_rules.contains_key(item) {
            seen_items.insert(item);
            continue;
        }

        let invalid_prev = invalid_rules.get(item).unwrap();
        for item in invalid_prev.iter() {
            if seen_items.contains(*item) {
                return false;
            }
        }
        seen_items.insert(item);
    }
    return true;
}

fn create_topological_sort(
    invalid_rules: &HashMap<&str, Vec<&str>>,
    valid_keys: &Vec<&str>,
) -> HashMap<String, u32> {
    let mut in_degree: HashMap<String, u32> = HashMap::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut result: HashMap<String, u32> = HashMap::new();

    // First, add all nodes to the graph (both from and to nodes)
    for (&from, to_list) in invalid_rules {
        let from_str = from.to_string();
        // Add 'from' node
        if valid_keys.contains(&from) {
            graph.entry(from_str.clone()).or_insert_with(Vec::new);
            in_degree.entry(from_str.clone()).or_insert(0);
        }

        // Add all 'to' nodes
        for &to in to_list {
            let to_str = to.to_string();
            if valid_keys.contains(&to) {
                graph.entry(to_str.clone()).or_insert_with(Vec::new);
                in_degree.entry(to_str.clone()).or_insert(0);
            }
        }
    }

    // Now build the edges
    for (&from, to_list) in invalid_rules {
        let from_str = from.to_string();
        for &to in to_list {
            let to_str = to.to_string();
            if valid_keys.contains(&from) && valid_keys.contains(&to) {
                graph.get_mut(&from_str).unwrap().push(to_str.clone());
                *in_degree.get_mut(&to_str).unwrap() += 1;
            }
        }
    }

    // Find nodes with 0 in-degree
    let mut queue: VecDeque<String> = VecDeque::new();
    for (node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node.clone());
        }
    }

    let mut order: u32 = 0;
    // Process queue
    while let Some(node) = queue.pop_front() {
        result.insert(node.clone(), order);
        order += 1;

        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                let in_deg = in_degree.get_mut(neighbor).unwrap();
                *in_deg -= 1;
                if *in_deg == 0 {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut invalid_rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut total: u32 = 0;
    for line in input.lines() {
        // Construct rules
        let rule: Vec<&str> = line.split("|").collect();
        if rule.len() > 1 {
            let prev: &str = rule[0];
            let next: &str = rule[1];
            if !invalid_rules.contains_key(prev) {
                invalid_rules.insert(prev, vec![]);
            }
            let invalid_prev = invalid_rules.get_mut(prev).unwrap();
            invalid_prev.push(next);
        }
    }

    for line in input.lines() {
        let list_items: Vec<&str> = line.split(",").collect();
        if list_items.len() > 1 {
            if !is_valid(&invalid_rules, &list_items) {
                // Sort items according to topological order
                let topological_sort = create_topological_sort(&invalid_rules, &list_items);
                let mut sorted_items: Vec<(&str, u32)> = list_items
                    .iter()
                    .map(|&item| (item, *topological_sort.get(item).unwrap_or(&0)))
                    .collect();
                sorted_items.sort_by_key(|&(_, order)| order);
                let sorted_list: Vec<&str> =
                    sorted_items.into_iter().map(|(item, _)| item).collect();
                let center_item = sorted_list[sorted_list.len() / 2].parse::<u32>().unwrap();
                total += center_item;
            }
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
