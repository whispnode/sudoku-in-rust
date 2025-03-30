use solver::backtrack;

mod cell;

pub struct Board {
    grid: [[cell::Cell; 9]; 9],
    pub error_cells: Vec<(u8, u8)>,
    pub is_solved: bool,
    pub mistakes: i32,
}

impl Board {
    pub fn new() -> Self {
        // Generate a completed grid of numbers using backtracking.
        let generate_board = backtrack::Generate::new();

        // Create a new board grid with default cells.
        let mut grid = [[cell::Cell::default(); 9]; 9];

        // Iterate over the indices to assign positions, values, and fixed flags.
        for i in 0..9 {
            for j in 0..9 {
                let value = generate_board.g_grid[i][j];
                grid[i][j] = cell::Cell {
                    position: (i as u8, j as u8),
                    value,
                    is_fixed: value != 0,
                };
            }
        }

        Board {
            grid,
            error_cells: Vec::new(),
            is_solved: false,
            mistakes: 0,
        }
    }

    pub fn display_grid(&self) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    print!("|  ");
                }
                print!("{}  ", cell.value);
            }
            println!();
            if (i + 1) % 3 == 0 && i != 8 {
                println!("---------+-----------+---------");
            }
        }
    }

    fn can_place(&self, row: u8, col: u8, num: u8) -> bool {
        let row = row as usize;
        let col = col as usize;

        // Check the row and column for repeats
        for i in 0..9 {
            if self.grid[row][i].value == num || self.grid[i][col].value == num {
                return false;
            }
        }

        // Determine the top left corner of the smaller 3x3 grid of the g_grid
        let box_row = row - (row % 3);
        let box_col = col - (col % 3);

        // Check the smaller 3x3 grids of the g_grid
        for i in 0..3 {
            for j in 0..3 {
                if self.grid[box_row + i][box_col + j].value == num {
                    return false;
                }
            }
        }

        true
    }

    fn check_errors(&mut self, value: u8, position: (u8, u8)) {
        let in_error_cells = self.error_cells.contains(&position);

        if value == 0 {
            if in_error_cells {
                self.error_cells.retain(|&e| e != position);
            }

            if self.grid[position.0 as usize][position.1 as usize].is_fixed {
                println!("Cell at {:?} is a fixed value", position);
            }
            return;
        }

        if self.can_place(position.0, position.1, value) {
            if in_error_cells {
                self.error_cells.retain(|&e| e != position);
            }
        } else {
            self.mistakes += 1;

            if !self.grid[position.0 as usize][position.1 as usize].is_fixed {
                if !in_error_cells {
                    self.error_cells.push(position);
                }
            }
        }
    }

    pub fn enter_num(&mut self, value: u8, position: (u8, u8)) {
        self.check_errors(value, position);

        if !self.grid[position.0 as usize][position.1 as usize].is_fixed {
            self.grid[position.0 as usize][position.1 as usize].value = value;
        } else {
            println!("\ncell at {:?} is fixed", position);
            return;
        }
    }

    pub fn has_filled_cells(&self) -> bool {
        for row in self.grid {
            for cell in row {
                if cell.value == 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_complete(&mut self) {
        if self.error_cells.is_empty() {
            if self.has_filled_cells() {
                self.is_solved = true;
            }
        }
    }
}
