use std::io::{self, Write};

mod board;

fn main() {
    let mut game = board::Board::new();

    game.display_grid();
    game.is_complete();

    while !game.is_solved {
        // Get position input
        print!("\nEnter position (e.g., 1,2): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }
        let input = input.trim();
        if input.is_empty() {
            println!("Position cannot be empty!");
            continue;
        }

        let cleaned_input: String = input
            .chars()
            .filter(|&c| c.is_digit(10) || c == ',')
            .collect();

        // Split the cleaned input by comma
        let parts: Vec<&str> = cleaned_input.split(',').collect();
        if parts.len() != 2 {
            println!("Invalid position! Enter two numbers separated by a comma.");
            return;
        }

        // Try to parse each part to u8
        let position: Result<Vec<u8>, _> = parts.iter().map(|&s| s.parse::<u8>()).collect();

        let position = match position {
            Ok(pos) => pos,
            Err(_) => {
                println!("Error parsing position. Please enter valid numbers.");
                continue;
            }
        };

        if position[0] > 8 || position[1] > 8 {
            println!("Position must be any number within 0 to 8 inclusive");
            continue;
        }

        // Get number input
        print!("Enter number: ");
        io::stdout().flush().unwrap();

        let mut number_input = String::new();
        if io::stdin().read_line(&mut number_input).is_err() {
            println!("Failed to read input.");
            continue;
        }
        let number_input = number_input.trim();

        let number: u8 = match number_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number! Please enter a valid number.");
                continue;
            }
        };

        game.enter_num(number, (position[0], position[1]));
        game.is_complete();

        if !game.error_cells.is_empty() {
            println!(
                "MISTAKES: {1}\tERROR CELLS: {:?}",
                game.error_cells, game.mistakes
            );
        } else {
            println!(
                "MISTAKES: {1}\tERROR CELLS: {:?}",
                game.error_cells, game.mistakes
            );
        }

        println!();

        game.display_grid();
    }

    if game.is_solved {
        println!("\nsudoku solved");
    }
}
