pub mod template;

// Use this file to add helper functions and additional modules.
pub fn print_grid<T: std::fmt::Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for cell in row {
            print!("{} ", cell);
        }
        println!(); // New line after each row
    }
}
