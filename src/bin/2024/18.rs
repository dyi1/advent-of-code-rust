// use advent_of_code::print_grid;
use std::collections::VecDeque;
advent_of_code::solution!(18);

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    steps: u32,
}

fn run_bfs(grid: Vec<Vec<char>>) -> Option<u32> {
    let length = grid.len();
    let width = grid[0].len();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; width]; length];

    // Run BFS on coordinates + steps
    let mut queue: VecDeque<Position> = VecDeque::new();
    queue.push_back(Position {
        x: 0,
        y: 0,
        steps: 0,
    });

    loop {
        if queue.is_empty() {
            return None;
        }
        let p = queue.pop_front().unwrap();
        if seen[p.x][p.y] {
            continue;
        }
        if p.x == length - 1 && p.y == width - 1 {
            // for row in seen {
            //     for item in row {
            //         if item {
            //             print!("t");
            //         } else {
            //             print!(" ");
            //         }
            //     }
            //     println!("");
            // }
            return Some(p.steps);
        }

        // Search thru space
        if p.x > 0 && !seen[p.x - 1][p.y] && grid[p.x - 1][p.y] != '#' {
            queue.push_back(Position {
                x: p.x - 1,
                y: p.y,
                steps: p.steps + 1,
            });
        }
        if p.x < length - 1 && !seen[p.x + 1][p.y] && grid[p.x + 1][p.y] != '#' {
            queue.push_back(Position {
                x: p.x + 1,
                y: p.y,
                steps: p.steps + 1,
            });
        }
        if p.y > 0 && !seen[p.x][p.y - 1] && grid[p.x][p.y - 1] != '#' {
            queue.push_back(Position {
                x: p.x,
                y: p.y - 1,
                steps: p.steps + 1,
            });
        }
        if p.y < width - 1 && !seen[p.x][p.y + 1] && grid[p.x][p.y + 1] != '#' {
            queue.push_back(Position {
                x: p.x,
                y: p.y + 1,
                steps: p.steps + 1,
            });
        }
        seen[p.x][p.y] = true;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let length = 71;
    let width = 71;

    let num_bytes = 1024;
    let bytes: Vec<&str> = input.lines().collect();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; length];
    for i in 0..num_bytes {
        if i >= bytes.len() {
            break;
        }
        let coordinates: Vec<&str> = bytes[i].split(",").collect();
        let x = coordinates[0].parse::<usize>().unwrap();
        let y = coordinates[1].parse::<usize>().unwrap();
        grid[x][y] = '#';
    }
    return run_bfs(grid);
}

pub fn part_two(input: &str) -> Option<String> {
    let length = 71;
    let width = 71;

    let bytes: Vec<&str> = input.lines().collect();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; length];
    for i in 0..bytes.len() {
        let coordinates: Vec<&str> = bytes[i].split(",").collect();
        let x = coordinates[0].parse::<usize>().unwrap();
        let y = coordinates[1].parse::<usize>().unwrap();
        grid[x][y] = '#';
        let min_steps = run_bfs(grid.clone());
        if min_steps.is_none() {
            return Some(bytes[i].to_string());
        }
    }

    return None;
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
