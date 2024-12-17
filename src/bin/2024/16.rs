advent_of_code::solution!(16);
use std::collections::HashSet;

use priority_queue::PriorityQueue;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Position {
    i: i32,
    j: i32,
    di: i32,
    dj: i32,
    steps: u32,
    turns: u32,
}

fn step_forward(p: Position) -> Position {
    return Position {
        i: p.i + p.di,
        j: p.j + p.dj,
        dj: p.dj,
        di: p.di,
        steps: p.steps + 1,
        turns: p.turns,
    };
}

fn turn_left(p: Position) -> Position {
    return Position {
        i: p.i,
        j: p.j,
        dj: -p.di,
        di: p.dj,
        steps: p.steps,
        turns: p.turns + 1,
    };
}

fn turn_right(p: Position) -> Position {
    return Position {
        i: p.i,
        j: p.j,
        dj: p.di,
        di: -p.dj,
        steps: p.steps,
        turns: p.turns + 1,
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut goal: (i32, i32) = (0, 0);
    let mut pq: PriorityQueue<Position, i32> = PriorityQueue::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                pq.push(
                    Position {
                        i: i as i32,
                        j: j as i32,
                        di: 0,
                        dj: 1,
                        steps: 0,
                        turns: 0,
                    },
                    0,
                );
            }
            if grid[i][j] == 'E' {
                goal = (i as i32, j as i32);
            }
        }
    }

    let (goal_i, goal_j) = goal;
    // println!("{}, {}", goal_i, goal_j);
    // let seen_pos: HashSet<Position> = HashSet::new();
    loop {
        let (current_pos, pri) = pq.pop().unwrap();
        // println!("{:?}, {}", current_pos, pri);
        if current_pos.i == goal_i && current_pos.j == goal_j {
            let min_score: u32 = (-1 * pri) as u32;
            println!("{:?}, {}", current_pos, pri);
            return Some(min_score);
        }
        let left_pos = turn_left(current_pos.clone());
        // if !seen_pos.contains(&left_pos) {
        pq.push(left_pos, pri - 1000);
        // }

        let right_pos = turn_right(current_pos.clone());
        // if !seen_pos.contains(&right_pos) {
        pq.push(right_pos, pri - 1000);
        // }

        let next_pos = step_forward(current_pos.clone());
        if grid[next_pos.i as usize][next_pos.j as usize] != '#' {
            pq.push(next_pos, pri - 1);
        }
    }
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
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
