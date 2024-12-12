advent_of_code::solution!(9);

#[derive(Clone)]
struct DiskBlock {
    file_id: Option<usize>, // None means space
    size: u32,
}

pub fn part_one(input: &str) -> Option<u64> {
    // Parse input into alternating file/space blocks
    let mut blocks: Vec<DiskBlock> = Vec::new();
    let mut file_id = 0;

    for (i, num) in input.chars().filter_map(|c| c.to_digit(10)).enumerate() {
        blocks.push(DiskBlock {
            file_id: if i % 2 == 0 { Some(file_id) } else { None },
            size: num,
        });
        if i % 2 == 0 {
            file_id += 1;
        }
    }

    // Simulate moving files left
    let final_positions = simulate_defrag(&mut blocks);

    // Calculate checksum
    let checksum = calculate_checksum(&final_positions);

    Some(checksum)
}

fn simulate_defrag(blocks: &mut [DiskBlock]) -> Vec<Option<usize>> {
    // Convert blocks to a position-by-position representation
    let mut positions: Vec<Option<usize>> = Vec::new();

    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = blocks.len() - 1;

    while left_pointer <= right_pointer {
        // println!("{}, {}", left_pointer, right_pointer);
        let mut left_disk_block = blocks[left_pointer].clone();
        if left_disk_block.size == 0 {
            left_pointer += 1;
            continue;
        }
        let mut right_disk_block = blocks[right_pointer].clone();
        if right_disk_block.size == 0 {
            right_pointer -= 2;
            continue;
        }
        match left_disk_block.file_id {
            Some(file_id) => {
                for _i in 0..left_disk_block.size {
                    positions.push(Some(file_id))
                }
                left_pointer += 1;
            }
            None => {
                positions.push(right_disk_block.file_id);
                right_disk_block.size -= 1;
                left_disk_block.size -= 1;
                blocks[right_pointer] = right_disk_block;
                blocks[left_pointer] = left_disk_block;
            }
        }
    }
    positions
}

fn calculate_checksum(positions: &[Option<usize>]) -> u64 {
    positions
        .iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| pos as u64 * id as u64))
        .sum()
}

fn simulate_defrag_pt2(blocks: &mut Vec<DiskBlock>) -> Vec<Option<usize>> {
    let mut positions: Vec<Option<usize>> = Vec::new();

    // Process from right to left
    let mut i = blocks.len() - 1;
    while i > 0 {
        if let Some(current_id) = blocks[i].file_id {
            let file_size = blocks[i].size;
            let mut space_needed = file_size;
            let mut insertion_idx = 0;

            // Look for leftmost suitable position
            while insertion_idx < i {
                if blocks[insertion_idx].file_id.is_none() {
                    let free_size = blocks[insertion_idx].size;

                    if free_size >= space_needed {
                        // Found enough space, move the file here
                        blocks[i] = DiskBlock {
                            file_id: None,
                            size: file_size,
                        };

                        // Update the target position
                        blocks[insertion_idx] = DiskBlock {
                            file_id: Some(current_id),
                            size: file_size,
                        };

                        // If there's remaining space, create a new free block
                        if free_size > file_size {
                            blocks.insert(
                                insertion_idx + 1,
                                DiskBlock {
                                    file_id: None,
                                    size: free_size - file_size,
                                },
                            );
                        }
                        break;
                    }
                }
                insertion_idx += 1;
            }
        }
        i -= 1;
    }

    // Build final positions vector
    for block in blocks.iter() {
        for _ in 0..block.size {
            positions.push(block.file_id);
        }
    }

    positions
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks: Vec<DiskBlock> = Vec::new();
    let mut file_id = 0;

    for (i, num) in input.chars().filter_map(|c| c.to_digit(10)).enumerate() {
        blocks.push(DiskBlock {
            file_id: if i % 2 == 0 { Some(file_id) } else { None },
            size: num,
        });
        if i % 2 == 0 {
            file_id += 1;
        }
    }

    // Simulate moving files left
    let final_positions = simulate_defrag_pt2(&mut blocks);

    // Calculate checksum
    let checksum = calculate_checksum(&final_positions);

    Some(checksum)
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
