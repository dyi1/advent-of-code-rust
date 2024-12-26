use std::collections::HashMap;

advent_of_code::solution!(24);

#[derive(Debug)]
struct Instruction {
    val1: String,
    val2: String,
    op: String,
    res: String,
}

fn topological_sort(dag: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut in_degree: HashMap<String, i32> = HashMap::new();
    let mut result = Vec::new();
    let mut queue = Vec::new();

    // Calculate in-degree for each node
    for (node, deps) in dag.iter() {
        // Ensure the node is in in_degree map
        in_degree.entry(node.clone()).or_insert(0);
        // Count incoming edges
        for dep in deps {
            *in_degree.entry(dep.clone()).or_insert(0) += 1;
        }
    }

    // Find nodes with no incoming edges
    for (node, &degree) in in_degree.iter() {
        if degree == 0 {
            queue.push(node.clone());
        }
    }

    // Process queue
    while let Some(node) = queue.pop() {
        result.push(node.clone());

        // Reduce in-degree for all neighbors
        if let Some(deps) = dag.get(&node) {
            for dep in deps {
                if let Some(degree) = in_degree.get_mut(dep) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(dep.clone());
                    }
                }
            }
        }
    }

    result.reverse(); // Reverse to get correct dependency order
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut wires: HashMap<String, u32> = HashMap::new();
    for w in sections[0].lines() {
        // println!("{}", w);
        let items: Vec<String> = w.split(":").map(|s| s.trim().to_string()).collect();
        let key = items[0].clone();
        let num = items[1].parse::<u32>().unwrap();
        wires.insert(key, num);
    }

    let mut dag: HashMap<String, Vec<String>> = HashMap::new();
    let mut instructions: HashMap<String, Instruction> = HashMap::new();

    for i in sections[1].lines() {
        let items: Vec<String> = i.split_whitespace().map(|s| s.trim().to_string()).collect();
        let res = items[4].clone();
        let val1 = items[0].clone();
        let op = items[1].clone();
        let val2 = items[2].clone();

        dag.insert(res.clone(), vec![val1.clone(), val2.clone()]);
        instructions.insert(
            res.clone(),
            Instruction {
                val1: val1,
                val2: val2,
                op: op,
                res: res,
            },
        );
    }

    let sorted_nodes = topological_sort(&dag);
    for node in sorted_nodes {
        if wires.contains_key(&node) {
            continue;
        }
        let inst = instructions.get(&node).unwrap();
        let val1 = wires.get(&inst.val1).unwrap();
        let val2 = wires.get(&inst.val2).unwrap();
        match inst.op.as_str() {
            "AND" => wires.insert(inst.res.clone(), val1 & val2),
            "XOR" => wires.insert(inst.res.clone(), val1 ^ val2),
            "OR" => wires.insert(inst.res.clone(), val1 | val2),
            _ => None,
        };
    }

    let mut z_wires: Vec<(&String, &u32)> =
        wires.iter().filter(|(k, _)| k.starts_with("z")).collect();
    z_wires.sort_by(|(a, _), (b, _)| {
        let a_num = a.trim_start_matches('z').parse::<u32>().unwrap_or(0);
        let b_num = b.trim_start_matches('z').parse::<u32>().unwrap_or(0);
        b_num.cmp(&a_num)
    });
    let mut total: u64 = 0;
    for z_wire in z_wires {
        total *= 2;
        total += *z_wire.1 as u64;
    }

    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut wires: HashMap<String, u32> = HashMap::new();
    for w in sections[0].lines() {
        // println!("{}", w);
        let items: Vec<String> = w.split(":").map(|s| s.trim().to_string()).collect();
        let key = items[0].clone();
        let num = items[1].parse::<u32>().unwrap();
        wires.insert(key, num);
    }

    let mut dag: HashMap<String, Vec<String>> = HashMap::new();
    let mut instructions: HashMap<String, Instruction> = HashMap::new();

    for i in sections[1].lines() {
        let items: Vec<String> = i.split_whitespace().map(|s| s.trim().to_string()).collect();
        let res = items[4].clone();
        let val1 = items[0].clone();
        let op = items[1].clone();
        let val2 = items[2].clone();

        dag.insert(res.clone(), vec![val1.clone(), val2.clone()]);
        instructions.insert(
            res.clone(),
            Instruction {
                val1: val1,
                val2: val2,
                op: op,
                res: res,
            },
        );
    }
    let test_keys = ["z00", "z01", "z02", "z03"];
    for t in test_keys {
        println!("{}", t);

        let mut next_keys_to_test = vec![t];
        let empty_vec = vec![];
        while next_keys_to_test.len() > 0 {
            let next_key = next_keys_to_test.pop().unwrap();
            let found = dag.get(next_key).unwrap_or(&empty_vec);
            for k in found {
                next_keys_to_test.push(k.as_str());
            }
            if instructions.get(next_key).is_some() {
                println!("{:?}", instructions.get(next_key));
            }
        }
    }

    return Some(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
