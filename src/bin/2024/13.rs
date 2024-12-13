use num::{integer::gcd, traits::float};

advent_of_code::solution!(13);

#[derive(Clone, Debug)]
struct Machine {
    a_button: (u64, u64),
    b_button: (u64, u64),
    goal: (u64, u64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut machine: Machine = Machine {
        a_button: (0, 0),
        b_button: (0, 0),
        goal: (0, 0),
    };
    let mut machines_list: Vec<Machine> = vec![];
    let mut mod_number = 1;

    // Parse Input
    for line in input.lines() {
        let items: Vec<&str> = line.split_whitespace().collect();

        if mod_number == 0 {
            machines_list.push(machine.clone());
            machine = Machine {
                a_button: (0, 0),
                b_button: (0, 0),
                goal: (0, 0),
            };
        }
        if mod_number == 1 || mod_number == 2 {
            let x_button: Vec<&str> = items[2].split("+").collect();
            let y_button: Vec<&str> = items[3].split("+").collect();
            let x_gain = x_button[1]
                .strip_suffix(',')
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let y_gain = y_button[1].parse::<u64>().unwrap();
            if mod_number == 1 {
                machine.a_button = (x_gain, y_gain);
            } else {
                machine.b_button = (x_gain, y_gain);
            }
        }
        if mod_number == 3 {
            let x_button: Vec<&str> = items[1].split("=").collect();
            let y_button: Vec<&str> = items[2].split("=").collect();
            let x_goal = x_button[1]
                .strip_suffix(',')
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let y_goal = y_button[1].parse::<u64>().unwrap();
            machine.goal = (x_goal, y_goal);
        }
        mod_number = (mod_number + 1) % 4
    }
    // Push one at end.
    machines_list.push(machine);

    let mut total_tokens_required: u64 = 0;
    for machine in machines_list {
        // if has_solution(&machine) {
        // println!("trying to solve {:?}", machine);
        total_tokens_required += solve_machine(&machine);
        // } else {
        //     println!("No solution supposedly {:?}", machine);
        // }
    }
    return Some(total_tokens_required);
}

fn _has_solution(machine: &Machine) -> bool {
    let (x_a, y_a) = machine.a_button;
    let (x_b, y_b) = machine.b_button;
    let (x_goal, y_goal) = machine.goal;
    // Solve x side
    let gcd_x = gcd(x_a, x_b);
    let gcd_y = gcd(y_a, y_b);
    return x_goal % gcd_x == 0 && y_goal % gcd_y == 0;
}

fn solve_machine(machine: &Machine) -> u64 {
    let (x_a, y_a) = machine.a_button;
    let (x_b, y_b) = machine.b_button;
    let (x_goal, y_goal) = machine.goal;

    // Solve with lin algebra
    let determinant = (x_a * y_b) as i64 - (y_a * x_b) as i64;
    if determinant == 0 {
        print!("failed here");
        return 0;
    }
    // Calculate numerators
    let num_a = (x_goal * y_b) as i64 - (y_goal * x_b) as i64;
    let num_b = (x_a * y_goal) as i64 - (y_a * x_goal) as i64;

    // Check if we have integer solutions
    if num_a % determinant != 0 || num_b % determinant != 0 {
        return 0;
    }

    let a_press = num_a / determinant;
    let b_press = num_b / determinant;
    if a_press < 0 || b_press < 0 {
        return 0;
    }
    if a_press as u64 * x_a + b_press as u64 * x_b != x_goal {
        return 0;
    }

    if a_press as u64 * y_a + b_press as u64 * y_b != y_goal {
        return 0;
    }
    return (a_press * 3 + b_press) as u64;
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut machine: Machine = Machine {
        a_button: (0, 0),
        b_button: (0, 0),
        goal: (0, 0),
    };
    let mut machines_list: Vec<Machine> = vec![];
    let mut mod_number = 1;
    let offset: u64 = 10000000000000;
    // Parse Input
    for line in input.lines() {
        let items: Vec<&str> = line.split_whitespace().collect();

        if mod_number == 0 {
            machines_list.push(machine.clone());
            machine = Machine {
                a_button: (0, 0),
                b_button: (0, 0),
                goal: (0, 0),
            };
        }
        if mod_number == 1 || mod_number == 2 {
            let x_button: Vec<&str> = items[2].split("+").collect();
            let y_button: Vec<&str> = items[3].split("+").collect();
            let x_gain = x_button[1]
                .strip_suffix(',')
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let y_gain = y_button[1].parse::<u64>().unwrap();
            if mod_number == 1 {
                machine.a_button = (x_gain, y_gain);
            } else {
                machine.b_button = (x_gain, y_gain);
            }
        }
        if mod_number == 3 {
            let x_button: Vec<&str> = items[1].split("=").collect();
            let y_button: Vec<&str> = items[2].split("=").collect();

            let x_goal: u64 = x_button[1]
                .strip_suffix(',')
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let y_goal: u64 = y_button[1].parse::<u64>().unwrap();
            machine.goal = (x_goal + offset, y_goal + offset);
        }
        mod_number = (mod_number + 1) % 4
    }
    // Push one at end.
    machines_list.push(machine);

    let mut total_tokens_required: u64 = 0;
    for machine in machines_list {
        // if has_solution(&machine) {
        total_tokens_required += solve_machine(&machine);
        // } else {
        //     println!("No solution supposedly {:?}", machine);
        // }
    }
    return Some(total_tokens_required);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
