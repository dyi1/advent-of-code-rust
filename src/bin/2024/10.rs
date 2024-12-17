use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

fn evaluate_trails(grid: &Vec<Vec<u32>>) -> (u32, u32) {
    let mut trail_visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut trail_count = vec![vec![0; grid[0].len()]; grid.len()];
    let mut trail_head: Vec<Vec<HashSet<(usize, usize)>>> =
        vec![vec![HashSet::new(); grid[0].len()]; grid.len()];

    let mut next_bfs: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                trail_count[i][j] += 1;
                trail_head[i][j].insert((i, j));
                next_bfs.push_back((i, j));
            }
        }
    }
    let mut part_1: u32 = 0;
    let mut part_2: u32 = 0;
    while !next_bfs.is_empty() {
        let (x, y) = next_bfs.pop_front().unwrap();
        if trail_visited[x][y] {
            continue;
        }
        let current_val = grid[x][y];
        if current_val == 9 {
            // println!("{:?}", trail_head[x][y]);
            part_1 += trail_head[x][y].len() as u32;
            part_2 += trail_count[x][y];
        }

        if x > 0 {
            if grid[x - 1][y] == current_val + 1 {
                trail_count[x - 1][y] += trail_count[x][y];
                for (i, j) in trail_head[x][y].clone() {
                    trail_head[x - 1][y].insert((i, j));
                }
                next_bfs.push_back((x - 1, y));
            }
        }
        if x < grid.len() - 1 {
            if grid[x + 1][y] == current_val + 1 {
                trail_count[x + 1][y] += trail_count[x][y];
                for (i, j) in trail_head[x][y].clone() {
                    trail_head[x + 1][y].insert((i, j));
                }

                next_bfs.push_back((x + 1, y));
            }
        }
        if y < grid[0].len() - 1 {
            if grid[x][y + 1] == current_val + 1 {
                trail_count[x][y + 1] += trail_count[x][y];
                for (i, j) in trail_head[x][y].clone() {
                    trail_head[x][y + 1].insert((i, j));
                }

                next_bfs.push_back((x, y + 1));
            }
        }
        if y > 0 {
            if grid[x][y - 1] == current_val + 1 {
                trail_count[x][y - 1] += trail_count[x][y];
                for (i, j) in trail_head[x][y].clone() {
                    trail_head[x][y - 1].insert((i, j));
                }
                next_bfs.push_back((x, y - 1));
            }
        }
        trail_visited[x][y] = true;
    }

    return (part_1, part_2);
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let (part_1, _) = evaluate_trails(&grid);
    return Some(part_1);
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let (_, part_2) = evaluate_trails(&grid);
    return Some(part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
