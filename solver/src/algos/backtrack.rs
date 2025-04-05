use super::utils::can_place;
use super::utils::remove_numbers;
use rand::seq::SliceRandom;

pub struct Generate {
    pub g_grid: [[u8; 9]; 9],
}

impl Generate {
    pub fn new() -> Self {
        let mut instance = Generate {
            g_grid: [[0u8; 9]; 9],
        };
        fill_grid(&mut instance.g_grid, 0, 0, true);
        remove_numbers(&mut instance.g_grid);
        instance
    }
}

// Recursive function to fill a grid with numbers using backtracking.
fn fill_grid(grid: &mut [[u8; 9]; 9], row: usize, col: usize, is_random_grid: bool) -> bool {
    // If we have filled all rows, the grid is complete.
    if row == 9 {
        return true;
    }

    // If we reach the end of a row, move to the next row.
    let (row, col) = if col == 9 { (row + 1, 0) } else { (row, col) };

    // If we have filled all rows after advancing, we're done.
    if row == 9 {
        return true;
    }

    // If cell is already filled (non-zero), skip to next cell.
    if grid[row][col] != 0 {
        return fill_grid(grid, row, col + 1, is_random_grid);
    }

    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    if is_random_grid {
        // Create a random number generator
        let mut rng = rand::rng();

        // Shuffle the vector
        numbers.shuffle(&mut rng);
    }

    // Try each number from 1 to 9.
    for num in numbers {
        if can_place(grid, row as u8, col as u8, num) {
            grid[row][col] = num;

            // Recursively try to fill the next cells
            if fill_grid(grid, row, col + 1, is_random_grid) {
                return true; // If the grid is successfully filled, return true
            }

            // Backtrack: If placing 'num' didn't work, reset the cell to 0 and try the next number
            grid[row][col] = 0;
        }
    }

    // Return false if no valid number can be placed in this cell.
    false
}

pub struct SolveGrid {
    pub g_grid: [[u8; 9]; 9],
}

impl SolveGrid {
    pub fn solve(grid: [[u8; 9]; 9]) -> Self {
        let mut instance = SolveGrid { g_grid: grid };
        fill_grid(&mut instance.g_grid, 0, 0, false);
        instance
    }
}
