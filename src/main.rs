use core::fmt;
use std::io::{stdin, stdout, Write};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

pub struct Game {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[None; 3]; 3],
            current_player: Player::X,
        }
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    fn print_board(&self) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if let Some(player) = cell {
                    match player {
                        Player::X => print!(" X "),
                        Player::O => print!(" O "),
                    }
                } else {
                    print!(" . ");
                }
                if j < 2 {
                    print!("|");
                }
            }
            println!();
            if i < 2 {
                println!("---|---|---");
            }
        }
    }

    fn get_coordinate(&self, coord_name: &str) -> usize {
        loop {
            let mut input = String::new();
            print!("Player {}: Enter {} number (1-3): ", self.current_player, coord_name);
            stdout().flush().unwrap();
            stdin().read_line(&mut input).expect("failed to read from stdin");
            match input.trim().parse::<usize>() {
                Ok(num) if num >= 1 && num <= 3 => return num - 1,
                _ => println!("Invalid input. Please enter a number between 1 and 3."),
            }
        }
    }

    fn get_coordinates(&self) -> (usize, usize) {
        loop {
            let x_coordinate = self.get_coordinate("row");
            let y_coordinate = self.get_coordinate("column");

            if self.board[x_coordinate][y_coordinate].is_some() {
                println!("The cell at ({}, {}) is already occupied. Please choose another cell.", x_coordinate + 1, y_coordinate + 1);
            } else {
                return (x_coordinate, y_coordinate);
            }
        }
    }

    fn check_winner(&self) -> Option<Player> {
        // Check rows and columns
        for i in 0..3 {
            if self.board[i][0].is_some()
                && self.board[i][0] == self.board[i][1]
                && self.board[i][1] == self.board[i][2] {
                    return self.board[i][0];
                }
            if self.board[0][i].is_some()
                && self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i] {
                    return self.board[0][i];
                }
        }

        // Check diagonals
        if self.board[0][0].is_some()
            && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2] {
                return self.board[0][0];
            }
        if self.board[0][2].is_some()
            && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0] {
                return self.board[0][2];
            }

        None
    }

    fn check_tie(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&cell| cell.is_some())) && self.check_winner().is_none()
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        game.print_board();
        let (x_coordinate, y_coordinate) = game.get_coordinates();
        game.board[x_coordinate][y_coordinate] = Some(game.current_player);

        if let Some(winner) = game.check_winner() {
            game.print_board();
            println!("Player {} wins!", winner);
            break;
        } else if game.check_tie() {
            game.print_board();
            println!("The game is a tie!");
            break;
        }

        game.switch_player();
    }
}
