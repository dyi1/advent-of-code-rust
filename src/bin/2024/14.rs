advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let row_bound = 101;
    let col_bound = 103;
    let mid_row = (row_bound - 1) / 2;
    let mid_col = (col_bound - 1) / 2;

    let mut q1: u32 = 0;
    let mut q2: u32 = 0;
    let mut q3: u32 = 0;
    let mut q4: u32 = 0;

    for line in input.lines() {
        let items: Vec<&str> = line.split_whitespace().collect();
        let (p, v) = (items[0], items[1]);
        let p_items: Vec<&str> = p.split("=").collect();
        let start_pos: Vec<i32> = p_items[1]
            .split(",")
            .map(|str| str.parse::<i32>().unwrap())
            .collect();

        let v_items: Vec<&str> = v.split("=").collect();
        let start_vel: Vec<i32> = v_items[1]
            .split(",")
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

        let mut x = (start_vel[0] * 100 + start_pos[0]) % row_bound;
        let mut y = (start_vel[1] * 100 + start_pos[1]) % col_bound;
        if x < 0 {
            x += row_bound;
        }
        if y < 0 {
            y += col_bound;
        }
        println!("{}, {}", x, y);
        if x < mid_row && y < mid_col {
            q1 += 1;
        }
        if x > mid_row && y < mid_col {
            q2 += 1;
        }
        if x < mid_row && y > mid_col {
            q3 += 1;
        }
        if x > mid_row && y > mid_col {
            q4 += 1;
        }
    }
    // println!("{}, {}, {}, {}", q1, q2, q3, q4);
    return Some(q1 * q2 * q3 * q4);
}

struct Robot {
    start_pos: Vec<i32>,
    start_vec: Vec<i32>,
}

pub fn part_two(input: &str) -> Option<u32> {
    let row_bound = 101;
    let col_bound = 103;
    let mid_row = (row_bound - 1) / 2;
    let mid_col = (col_bound - 1) / 2;

    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let items: Vec<&str> = line.split_whitespace().collect();
        let (p, v) = (items[0], items[1]);
        let p_items: Vec<&str> = p.split("=").collect();
        let start_pos: Vec<i32> = p_items[1]
            .split(",")
            .map(|str| str.parse::<i32>().unwrap())
            .collect();

        let v_items: Vec<&str> = v.split("=").collect();
        let start_vel: Vec<i32> = v_items[1]
            .split(",")
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

        robots.push(Robot {
            start_pos: start_pos,
            start_vec: start_vel,
        });
    }
    let mut center_count: u32 = 0;
    let mut index: u32 = 0;
    let delta = 10;
    for i in 0..10000 {
        let mut current = 0;
        let mut grid = vec![vec!['.'; row_bound as usize]; col_bound as usize];
        for r in &robots {
            let mut x = (r.start_vec[0] * i + r.start_pos[0]) % row_bound;
            let mut y = (r.start_vec[1] * i + r.start_pos[1]) % col_bound;
            if x < 0 {
                x += row_bound;
            }
            if y < 0 {
                y += col_bound;
            }
            grid[y as usize][x as usize] = '#';

            if x > mid_row - delta
                && y > mid_col - delta
                && y < mid_col + delta
                && x < mid_row + delta
            {
                current += 1;
            }
        }
        if current >= center_count {
            center_count = current;
            index = i as u32;
            println!("{}, {}", current, i);
            print_grid(&grid);
        }
    }

    return Some(index);
}

fn print_grid<T: std::fmt::Display>(grid: &Vec<Vec<T>>) {
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
