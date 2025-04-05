use crate::cell::Cell;
use rand::Rng;

pub trait CellValue {
    fn get_value(&self) -> u8;
}

impl CellValue for u8 {
    fn get_value(&self) -> u8 {
        *self
    }
}

impl CellValue for Cell {
    fn get_value(&self) -> u8 {
        self.value
    }
}

// Check if a num can be placed in grid[row][col]
pub fn can_place<T: CellValue>(grid: &[[T; 9]; 9], row: u8, col: u8, num: u8) -> bool {
    let row = row as usize;
    let col = col as usize;

    // Check the row and column for repeats
    for i in 0..9 {
        if grid[row][i].get_value() == num || grid[i][col].get_value() == num {
            return false;
        }
    }

    // Determine the top left corner of the smaller 3x3 grid
    let box_row = row - (row % 3);
    let box_col = col - (col % 3);

    // Check the smaller 3x3 grids
    for i in 0..3 {
        for j in 0..3 {
            if grid[box_row + i][box_col + j].get_value() == num {
                return false;
            }
        }
    }

    true
}

// This will NOT result in unique solution all the time
pub(crate) fn remove_numbers(grid: &mut [[u8; 9]; 9]) {
    let mut cells_to_remove: u8 = rand::rng().random_range(35..=55);

    while cells_to_remove > 0 {
        let row: usize = rand::rng().random_range(0..=8);
        let col: usize = rand::rng().random_range(0..=8);

        if grid[row][col] != 0 {
            grid[row as usize][col as usize] = 0;
            cells_to_remove -= 1;
        }
    }
}
