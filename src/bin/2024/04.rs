advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut word_grid: Vec<Vec<char>> = vec![];
    let mut rows_count: usize = 0;
    let mut col_count: usize = 0;
    let mut num_xmas = 0;
    for line in input.lines() {
        rows_count += 1;
        let chars: Vec<char> = line.chars().collect();
        col_count = chars.len();
        word_grid.push(chars.clone());
    }
    println!("{}, {}", rows_count, col_count);
    for i in 0..rows_count {
        for j in 0..col_count {
            // side by side
            if j <= col_count - 4 {
                if word_grid[i][j] == 'X'
                    && word_grid[i][j + 1] == 'M'
                    && word_grid[i][j + 2] == 'A'
                    && word_grid[i][j + 3] == 'S'
                {
                    num_xmas += 1;
                }
            }
            if j >= 3 {
                if word_grid[i][j] == 'X'
                    && word_grid[i][j - 1] == 'M'
                    && word_grid[i][j - 2] == 'A'
                    && word_grid[i][j - 3] == 'S'
                {
                    num_xmas += 1;
                }
            }
            // top down
            if i <= rows_count - 4 {
                if word_grid[i][j] == 'X'
                    && word_grid[i + 1][j] == 'M'
                    && word_grid[i + 2][j] == 'A'
                    && word_grid[i + 3][j] == 'S'
                {
                    num_xmas += 1;
                }
            }
            if i >= 3 {
                if word_grid[i][j] == 'X'
                    && word_grid[i - 1][j] == 'M'
                    && word_grid[i - 2][j] == 'A'
                    && word_grid[i - 3][j] == 'S'
                {
                    num_xmas += 1;
                }
            }

            // diagnol top left to bot right
            if i <= rows_count - 4 && j <= col_count - 4 {
                if word_grid[i][j] == 'X'
                    && word_grid[i + 1][j + 1] == 'M'
                    && word_grid[i + 2][j + 2] == 'A'
                    && word_grid[i + 3][j + 3] == 'S'
                {
                    num_xmas += 1;
                }
            }
            if i >= 3 && j >= 3 {
                if word_grid[i][j] == 'X'
                    && word_grid[i - 1][j - 1] == 'M'
                    && word_grid[i - 2][j - 2] == 'A'
                    && word_grid[i - 3][j - 3] == 'S'
                {
                    num_xmas += 1;
                }
            }
            // diagnol top right to bot left.
            if i <= rows_count - 4 && j >= 3 {
                if word_grid[i][j] == 'X'
                    && word_grid[i + 1][j - 1] == 'M'
                    && word_grid[i + 2][j - 2] == 'A'
                    && word_grid[i + 3][j - 3] == 'S'
                {
                    num_xmas += 1;
                }
            }
            if i >= 3 && j <= col_count - 4 {
                if word_grid[i][j] == 'X'
                    && word_grid[i - 1][j + 1] == 'M'
                    && word_grid[i - 2][j + 2] == 'A'
                    && word_grid[i - 3][j + 3] == 'S'
                {
                    num_xmas += 1;
                }
            }
        }
    }

    Some(num_xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut word_grid: Vec<Vec<char>> = vec![];
    let mut row_count: usize = 0;
    let mut col_count: usize = 0;
    let mut num_xmas = 0;
    for line in input.lines() {
        row_count += 1;
        let chars: Vec<char> = line.chars().collect();
        col_count = chars.len();
        word_grid.push(chars.clone());
    }

    for i in 1..row_count - 1 {
        for j in 1..col_count - 1 {
            if word_grid[i][j] == 'A' {
                if word_grid[i - 1][j - 1] == 'M'
                    && word_grid[i + 1][j + 1] == 'S'
                    && word_grid[i + 1][j - 1] == 'M'
                    && word_grid[i - 1][j + 1] == 'S'
                {
                    num_xmas += 1;
                }
                if word_grid[i - 1][j - 1] == 'S'
                    && word_grid[i + 1][j + 1] == 'M'
                    && word_grid[i + 1][j - 1] == 'S'
                    && word_grid[i - 1][j + 1] == 'M'
                {
                    num_xmas += 1;
                }
                if word_grid[i - 1][j - 1] == 'M'
                    && word_grid[i + 1][j + 1] == 'S'
                    && word_grid[i + 1][j - 1] == 'S'
                    && word_grid[i - 1][j + 1] == 'M'
                {
                    num_xmas += 1;
                }
                if word_grid[i - 1][j - 1] == 'S'
                    && word_grid[i + 1][j + 1] == 'M'
                    && word_grid[i + 1][j - 1] == 'M'
                    && word_grid[i - 1][j + 1] == 'S'
                {
                    num_xmas += 1;
                }
            }
        }
    }
    return Some(num_xmas);
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
