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
        println!("{}, {}", left_pointer, right_pointer);
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
