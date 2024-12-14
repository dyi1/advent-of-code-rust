use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut unique_points: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut row_counter = 0;
    let mut col_counter = 0;
    for line in input.lines() {
        col_counter = 0;
        let mut row_grid: Vec<char> = vec![];
        for c in line.chars() {
            row_grid.push(c);
            if c != '.' {
                if !unique_points.contains_key(&c) {
                    unique_points.insert(c, vec![]);
                }
                if let Some(list_of_points) = unique_points.get_mut(&c) {
                    list_of_points.push((row_counter, col_counter));
                }
            }
            col_counter += 1;
        }
        grid.push(row_grid);
        row_counter += 1;
    }
    let mut bool_checker = vec![vec![false; col_counter]; row_counter];
    let mut total: u32 = 0;
    for key in unique_points.keys() {
        let points = unique_points[key].clone();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let (p1_x, p1_y) = points[i];
                let (p2_x, p2_y) = points[j];

                let x_diff: i32 = p2_x as i32 - p1_x as i32;
                let y_diff: i32 = p2_y as i32 - p1_y as i32;

                let (n1_x, n1_y) = (p1_x as i32 - x_diff, p1_y as i32 - y_diff);
                let (n2_x, n2_y) = (p2_x as i32 + x_diff, p2_y as i32 + y_diff);
                if 0 <= n1_x
                    && n1_x < row_counter as i32
                    && 0 <= n1_y
                    && n1_y < col_counter as i32
                    && bool_checker[n1_x as usize][n1_y as usize] == false
                {
                    bool_checker[n1_x as usize][n1_y as usize] = true;
                    total += 1;
                }
                if 0 <= n2_x
                    && n2_x < row_counter as i32
                    && 0 <= n2_y
                    && n2_y < col_counter as i32
                    && bool_checker[n2_x as usize][n2_y as usize] == false
                {
                    bool_checker[n2_x as usize][n2_y as usize] = true;
                    total += 1;
                }
            }
        }
    }
    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut unique_points: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut row_counter = 0;
    let mut col_counter = 0;
    for line in input.lines() {
        col_counter = 0;
        let mut row_grid: Vec<char> = vec![];
        for c in line.chars() {
            row_grid.push(c);
            if c != '.' {
                if !unique_points.contains_key(&c) {
                    unique_points.insert(c, vec![]);
                }
                if let Some(list_of_points) = unique_points.get_mut(&c) {
                    list_of_points.push((row_counter, col_counter));
                }
            }
            col_counter += 1;
        }
        grid.push(row_grid);
        row_counter += 1;
    }
    let mut bool_checker = vec![vec![false; col_counter]; row_counter];
    let mut total: u32 = 0;

    for key in unique_points.keys() {
        let points = unique_points[key].clone();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let (p1_x, p1_y) = points[i];
                let (p2_x, p2_y) = points[j];

                let x_diff: i32 = p2_x as i32 - p1_x as i32;
                let y_diff: i32 = p2_y as i32 - p1_y as i32;
                let mut done = false;
                let mut multiplier = 0;
                while !done {
                    let (n1_x, n1_y) = (
                        p1_x as i32 - x_diff * multiplier,
                        p1_y as i32 - y_diff * multiplier,
                    );
                    let (n2_x, n2_y) = (
                        p2_x as i32 + x_diff * multiplier,
                        p2_y as i32 + y_diff * multiplier,
                    );
                    let valid_up = 0 <= n1_x
                        && n1_x < row_counter as i32
                        && 0 <= n1_y
                        && n1_y < col_counter as i32;
                    let valid_down: bool = 0 <= n2_x
                        && n2_x < row_counter as i32
                        && 0 <= n2_y
                        && n2_y < col_counter as i32;

                    if !valid_up && !valid_down {
                        done = true;
                    } else {
                        if valid_up && bool_checker[n1_x as usize][n1_y as usize] == false {
                            bool_checker[n1_x as usize][n1_y as usize] = true;
                            grid[n1_x as usize][n1_y as usize] = '#';
                            // println!("{}_{}", n1_x, n1_y);
                            total += 1;
                        }
                        if valid_down && bool_checker[n2_x as usize][n2_y as usize] == false {
                            bool_checker[n2_x as usize][n2_y as usize] = true;
                            grid[n2_x as usize][n2_y as usize] = '#';
                            total += 1;
                            // println!("{}_{}", n2_x, n2_y);
                        }
                        multiplier += 1;
                    }
                }
            }
        }
    }
    // print_grid(&grid);
    // print_grid(&bool_checker);

    return Some(total);
}

fn _print_grid<T: std::fmt::Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for cell in row {
            print!("{} ", cell);
        }
        println!(); // New line after each row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
