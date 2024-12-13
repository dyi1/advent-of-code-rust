use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut visisted: Vec<Vec<bool>> = vec![];
    let mut current_position: Option<(usize, usize)> = None;
    let mut row_counter = 0;
    let mut col_counter = 0;

    for line in input.lines() {
        let mut grid_row: Vec<char> = vec![];
        let mut visited_row: Vec<bool> = vec![];

        col_counter = 0;
        for c in line.chars() {
            if c == '^' {
                current_position = Some((row_counter, col_counter));
                grid_row.push('.');
            } else {
                grid_row.push(c);
            }
            visited_row.push(false);
            col_counter += 1;
        }

        grid.push(grid_row);
        visisted.push(visited_row);

        row_counter += 1;
    }

    if current_position.is_none() {
        return None;
    }
    // println!("{:?}", current_position);
    let mut dx: i32 = -1;
    let mut dy: i32 = 0;
    let mut finished = false;
    while !finished {
        let (x, y) = current_position.unwrap();
        visisted[x][y] = true;
        let next_x: i32 = x as i32 + dx;
        let next_y: i32 = y as i32 + dy;
        // println!("{}_{}, {}, {}", x, y, row_counter, col_counter);
        if next_x < 0 || next_x >= row_counter as i32 || next_y >= col_counter as i32 || next_y < 0
        {
            finished = true;
        } else if grid[next_x as usize][next_y as usize] != '#' {
            current_position = Some((next_x as usize, next_y as usize));
        } else {
            (dx, dy) = (dy, -dx)
        }
    }

    // println!("{:#?}", visisted);

    let mut total: u32 = 0;
    for row in visisted {
        for is_visited in row {
            if is_visited {
                total += 1;
            }
        }
    }
    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut visited: Vec<Vec<bool>> = vec![];
    let mut current_position: Option<(usize, usize)> = None;

    let mut row_counter = 0;
    let mut col_counter = 0;

    for line in input.lines() {
        let mut grid_row: Vec<char> = vec![];
        let mut visited_row: Vec<bool> = vec![];

        col_counter = 0;
        for c in line.chars() {
            if c == '^' {
                current_position = Some((row_counter, col_counter));
                grid_row.push('.');
            } else {
                grid_row.push(c);
            }
            visited_row.push(false);
            col_counter += 1;
        }

        grid.push(grid_row);
        visited.push(visited_row);

        row_counter += 1;
    }

    if current_position.is_none() {
        return None;
    }

    let mut dx: i32 = -1;
    let mut dy: i32 = 0;
    let mut finished = false;
    let initial_postion = current_position.unwrap().clone();
    println!("inital pos, {:?}", initial_postion);
    while !finished {
        let (x, y) = current_position.unwrap();
        visited[x][y] = true;
        let next_x: i32 = x as i32 + dx;
        let next_y: i32 = y as i32 + dy;
        // println!("{}_{}, {}, {}", x, y, row_counter, col_counter);
        if next_x < 0 || next_x >= row_counter as i32 || next_y >= col_counter as i32 || next_y < 0
        {
            finished = true;
        } else if grid[next_x as usize][next_y as usize] != '#' {
            current_position = Some((next_x as usize, next_y as usize));
        } else {
            (dx, dy) = (dy, -dx)
        }
    }

    let mut total: u32 = 0;
    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            if visited[i][j] && (i, j) != initial_postion {
                // println!("hit here {}_{}", i, j);
                let mut visited_state: HashSet<(usize, usize, i32, i32)> = HashSet::new();
                let mut moded_grid = grid.clone();
                moded_grid[i][j] = 'O';

                // Reset looping.
                let mut dx: i32 = -1;
                let mut dy: i32 = 0;
                current_position = Some(initial_postion);
                finished = false;

                while !finished {
                    let (x, y) = current_position.unwrap();
                    let current_state = (x, y, dx, dy);

                    if visited_state.contains(&current_state) {
                        finished = true;
                        total += 1;
                    } else {
                        visited_state.insert(current_state);
                    }
                    let next_x: i32 = x as i32 + dx;
                    let next_y: i32 = y as i32 + dy;
                    if next_x < 0
                        || next_x >= row_counter as i32
                        || next_y >= col_counter as i32
                        || next_y < 0
                    {
                        finished = true;
                    } else if moded_grid[next_x as usize][next_y as usize] == '.' {
                        current_position = Some((next_x as usize, next_y as usize));
                    } else {
                        (dx, dy) = (dy, -dx)
                    }
                }
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
