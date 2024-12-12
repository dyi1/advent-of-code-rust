use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut letter_grid: Vec<Vec<char>> = vec![];
    let mut seen_region: Vec<Vec<bool>> = vec![];

    let mut row_count: usize = 0;
    let mut col_count: usize = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        col_count = chars.len();
        letter_grid.push(chars);
        seen_region.push(vec![false; col_count]);
        row_count += 1;
    }

    let mut total: u32 = 0;
    for x in 0..row_count {
        for y in 0..col_count {
            if seen_region[x][y] {
                continue;
            }

            let curr_char = letter_grid[x][y];

            let mut area_counter: u32 = 0;
            let mut perimeter_counter: u32 = 0;
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back((x, y));
            while queue.len() > 0 {
                let (i, j) = queue.pop_front().unwrap();
                if seen_region[i][j] {
                    continue;
                }

                // Increase area by 1
                area_counter += 1;
                // Update perimeter if around border
                if i == 0 || curr_char != letter_grid[i - 1][j] {
                    perimeter_counter += 1;
                } else if !seen_region[i - 1][j] {
                    queue.push_back((i - 1, j));
                }

                if i == row_count - 1 || curr_char != letter_grid[i + 1][j] {
                    perimeter_counter += 1;
                } else if !seen_region[i + 1][j] {
                    queue.push_back((i + 1, j));
                }

                if j == 0 || curr_char != letter_grid[i][j - 1] {
                    perimeter_counter += 1;
                } else if !seen_region[i][j - 1] {
                    queue.push_back((i, j - 1));
                }

                if j == col_count - 1 || curr_char != letter_grid[i][j + 1] {
                    perimeter_counter += 1;
                } else if !seen_region[i][j + 1] {
                    queue.push_back((i, j + 1));
                }

                seen_region[i][j] = true;
            }

            total += perimeter_counter * area_counter;
        }
    }

    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
