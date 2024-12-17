advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut reg_a: u64 = 0;
    let mut reg_b: u64 = 0;
    let mut reg_c: u64 = 0;
    let mut program: Vec<&str> = vec![];
    for i in 0..5 {
        if i == 3 {
            continue;
        }
        let current_line = lines[i];
        let line_items: Vec<&str> = current_line.split_whitespace().collect();
        let key_item = line_items[line_items.len() - 1];
        if i < 3 {
            if i == 0 {
                reg_a = key_item.parse::<u64>().unwrap();
            }
            if i == 1 {
                reg_b = key_item.parse::<u64>().unwrap();
            }
            if i == 2 {
                reg_c = key_item.parse::<u64>().unwrap();
            }
        } else {
            program = key_item.split(",").collect();
        }
    }

    // Hardcode in tests
    // reg_a = 2024;
    // program = vec!["0", "1", "5", "4", "3", "0"];
    let output_values = run_program(reg_a, reg_b, reg_c, &program);
    return Some(output_values.join(","));
}

fn run_program(reg_a: u64, reg_b: u64, reg_c: u64, program: &Vec<&str>) -> Vec<String> {
    let mut inst_ptr: usize = 0;
    let mut output_values: Vec<String> = vec![];

    let mut reg_a = reg_a;
    let mut reg_b = reg_b;
    let mut reg_c = reg_c;
    loop {
        if inst_ptr >= program.len() {
            break;
        }
        let instruction = program[inst_ptr];
        let operand = program[inst_ptr + 1];
        let lit_operand: u64 = operand.parse::<u64>().unwrap();
        let combo_operand: u64 = match operand {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => reg_a,
            "5" => reg_b,
            "6" => reg_c,
            _ => 0,
        };
        inst_ptr += 2;
        match instruction {
            // adv
            "0" => {
                let num = reg_a;
                let denom = 2_u32.pow(combo_operand as u32) as u64;
                reg_a = num / denom;
            }
            // bxl
            "1" => {
                reg_b = reg_b ^ lit_operand;
            }
            // bst
            "2" => {
                reg_b = combo_operand % 8;
            }
            // jmp
            "3" => {
                if reg_a == 0 {
                    continue;
                }
                inst_ptr = lit_operand as usize;
            }
            // bitwise xor
            "4" => {
                reg_b = reg_b ^ reg_c;
            }
            // out
            "5" => output_values.push((combo_operand % 8).to_string()),
            // bdv
            "6" => {
                let num = reg_a;
                let denom: u64 = 2_u64.pow(combo_operand as u32) as u64;
                reg_b = num / denom;
            }
            // cdv
            "7" => {
                let num = reg_a;
                let denom: u64 = 2_u64.pow(combo_operand as u32) as u64;
                reg_c = num / denom;
            }
            _ => {
                continue;
            }
        }
    }
    return output_values;
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    // let mut reg_a: u64 = 0;
    // let mut reg_b: u64 = 0;
    // let mut reg_c: u64 = 0;
    let mut program: Vec<&str> = vec![];
    for i in 0..5 {
        if i == 3 {
            continue;
        }
        let current_line = lines[i];
        let line_items: Vec<&str> = current_line.split_whitespace().collect();
        let key_item = line_items[line_items.len() - 1];
        if i < 3 {
            if i == 0 || i == 1 || i == 2 {
                continue;
            }
        } else {
            program = key_item.split(",").collect();
        }
    }

    println!("{:?}", program);
    let mut exponent = program.len() - 1;

    // Track the value at each position
    let mut position_values: Vec<u64> = vec![0; program.len()];
    position_values[exponent] = 1;

    let mut guess: u64 = 8_u64.pow(exponent as u32);

    let mut counter = 0;
    let stopper = 1000;

    let mut output_values: Vec<String> = vec![];
    loop {
        if counter > stopper {
            break;
        }

        output_values = run_program(guess, 0, 0, &program);
        // println!("{}, {} : {:?}", counter, guess, output_values);

        if output_values == program {
            break;
        }

        if program[exponent].to_string() == output_values[exponent] {
            // Current position matches, move to next position
            exponent -= 1;
        } else {
            // Current value didn't work, try next value at this position
            position_values[exponent] += 1;

            while position_values[exponent] >= 8 {
                // We've tried all values at this position, need to backtrack
                position_values[exponent] = 0;
                if exponent + 1 < program.len() {
                    position_values[exponent + 1] += 1;
                    exponent += 1;
                }
            }

            // Recalculate guess based on all position values
            guess = 0;
            for (pos, &val) in position_values.iter().enumerate() {
                guess += val * 8_u64.pow(pos as u32);
            }
        }

        counter += 1;
    }
    println!("{}, {} : {:?}", counter, guess, output_values);
    return Some(guess);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let exp_result: String = "4,6,3,5,6,3,5,2,1,0".to_string();
        assert_eq!(result, Some(exp_result));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
