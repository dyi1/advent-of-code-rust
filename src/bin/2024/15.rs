use advent_of_code::print_grid;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut grid: Vec<Vec<char>> = sections[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut curr_position: (i32, i32) = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                curr_position = (x as i32, y as i32);
            }
        }
    }

    let instructions = sections[1].lines().collect::<Vec<&str>>().join("");
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    // print_grid(&grid);

    for c in instructions.chars() {
        if c == '^' {
            dy = 1;
            dx = 0;
        } else if c == '<' {
            dx = -1;
            dy = 0;
        } else if c == '>' {
            dx = 1;
            dy = 0;
        } else if c == 'v' {
            dy = -1;
            dx = 0;
        }
        let (x, y) = curr_position;
        let new_x = x + dx;
        let new_y = y - dy;

        let pos_x = new_x as usize;
        let pos_y = new_y as usize;

        if grid[pos_y][pos_x] == '.' {
            swap_characters(x, y, new_x, new_y, &mut grid);
            curr_position = (new_x, new_y);
        } else if grid[pos_y][pos_x] == '#' {
            continue;
        } else if grid[pos_y][pos_x] == 'O' {
            let mut found_block_push = false;
            let (mut swap_x, mut swap_y) = (new_x, new_y);
            while swap_x >= 0
                && swap_x as usize <= grid[0].len() - 1
                && swap_y >= 0
                && swap_y as usize <= grid.len() - 1
            {
                if grid[swap_y as usize][swap_x as usize] == '.' {
                    swap_characters(swap_x as i32, swap_y as i32, new_x, new_y, &mut grid);
                    found_block_push = true;
                    break;
                } else if grid[swap_y as usize][swap_x as usize] == '#' {
                    break;
                } else {
                    swap_x = swap_x + dx;
                    swap_y = swap_y - dy;
                }
            }

            // Update the @ character position at the end
            if found_block_push {
                swap_characters(x, y, new_x, new_y, &mut grid);
                curr_position = (new_x, new_y);
            }
        }
        // println!("Move {}", c);
        // print_grid(&grid);
    }

    print_grid(&grid);
    let mut total: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                total += (100 * y + x) as u32;
            }
        }
    }
    return Some(total);
}

fn swap_characters(old_x: i32, old_y: i32, new_x: i32, new_y: i32, grid: &mut Vec<Vec<char>>) {
    let old_x = old_x as usize;
    let old_y = old_y as usize;
    let old_char = grid[old_y][old_x];
    let new_x = new_x as usize;
    let new_y = new_y as usize;
    let new_char = grid[new_y][new_x];

    grid[new_y][new_x] = old_char;
    grid[old_y][old_x] = new_char;
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
