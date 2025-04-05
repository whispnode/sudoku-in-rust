// Check if a num can be placed in grid[row][col]
pub fn can_place(grid: &[[u8; 9]; 9], row: u8, col: u8, num: u8) -> bool {
    let row = row as usize;
    let col = col as usize;

    // Check the row and column for repeats
    for i in 0..9 {
        if grid[row][i] == num || grid[i][col] == num {
            return false;
        }
    }

    // Determine the top left corner of the smaller 3x3 grid
    let box_row = row - (row % 3);
    let box_col = col - (col % 3);

    // Check the smaller 3x3 grids
    for i in 0..3 {
        for j in 0..3 {
            if grid[box_row + i][box_col + j] == num {
                return false;
            }
        }
    }

    true
}
