use rand::Rng;
use rand::seq::SliceRandom; // Import shuffle trait

pub struct Generate {
    pub g_grid: [[u8; 9]; 9],
}

impl Generate {
    pub fn new() -> Self {
        let mut instance = Generate {
            g_grid: [[0u8; 9]; 9],
        };
        instance.fill_grid(0, 0);
        instance.remove_numbers();
        instance
    }

    // Check if a num can be placed in self.g_grid[row][col]
    fn can_place(&self, row: u8, col: u8, num: u8) -> bool {
        let row = row as usize;
        let col = col as usize;

        // Check the row and column for repeats
        for i in 0..9 {
            if self.g_grid[row][i] == num || self.g_grid[i][col] == num {
                return false;
            }
        }

        // Determine the top left corner of the smaller 3x3 grid of the g_grid
        let box_row = row - (row % 3);
        let box_col = col - (col % 3);

        // Check the smaller 3x3 grids of the g_grid
        for i in 0..3 {
            for j in 0..3 {
                if self.g_grid[box_row + i][box_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    // Recursive function to fill a grid with numbers using backtracking.
    fn fill_grid(&mut self, row: usize, col: usize) -> bool {
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
        if self.g_grid[row][col] != 0 {
            return self.fill_grid(row, col + 1);
        }

        let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Create a random number generator
        let mut rng = rand::rng();

        // Shuffle the vector
        numbers.shuffle(&mut rng);

        // Try each number from 1 to 9.
        for num in numbers {
            if self.can_place(row as u8, col as u8, num) {
                self.g_grid[row][col] = num;

                // Recursively try to fill the next cells
                if self.fill_grid(row, col + 1) {
                    return true; // If the grid is successfully filled, return true
                }

                // Backtrack: If placing 'num' didn't work, reset the cell to 0 and try the next number
                self.g_grid[row][col] = 0;
            }
        }

        // Return false if no valid number can be placed in this cell.
        false
    }

    // This will NOT result in unique solution all the time
    fn remove_numbers(&mut self) {
        let mut cells_to_remove: u8 = rand::rng().random_range(35..=55);

        while cells_to_remove > 0 {
            let row: usize = rand::rng().random_range(0..=8);
            let col: usize = rand::rng().random_range(0..=8);

            if self.g_grid[row][col] != 0 {
                self.g_grid[row as usize][col as usize] = 0;
                cells_to_remove -= 1;
            }
        }
    }
}

pub struct SolveGrid {
    g_grid: [[u8; 9]; 9],
}

impl SolveGrid {
    pub fn solve(grid: [[u8; 9]; 9]) -> Self {
        let instance = SolveGrid { g_grid: grid };
        instance
    }
}
