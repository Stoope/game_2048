use crossterm::{InputEvent, KeyEvent};
use std::io::{Error, ErrorKind};
mod console;

pub struct Board {
    board: Vec<i32>,
}

impl Board {
    pub fn new(board_size: i16) -> Result<Board, Error> {
        match board_size > 1 {
            true => {
                let board: Vec<i32> = vec![0; board_size.pow(2) as usize];

                Ok(Board { board })
            }
            false => Err(Error::new(
                ErrorKind::InvalidInput,
                "Board size must be greater then 1!",
            )),
        }
    }

    pub fn perform_action(&self, key_event: &InputEvent) {
        match key_event {
            InputEvent::Keyboard(k) => match k {
                KeyEvent::Char(c) => match c {
                    'w' => {
                        console::print_board(&self.board).expect("Error while processing board!");
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }

    fn move_right(&self) {}
}

pub fn board(board_size: i16) -> Result<Board, Error> {
    Board::new(board_size)
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn should_return_error_with_board_size_0() -> Result<(), ()> {
        match board(0) {
            Ok(_) => Err(()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn should_init_with_two_2() {
        let board_size = 6;

        match board(board_size) {
            Ok(board) => {
                let zero_count = board.board.iter().filter(|&item| *item == 0).count();
                let two_count = board.board.iter().filter(|&item| *item == 2).count();
                assert_eq!(zero_count, board_size.pow(2) as usize - 2);
                assert_eq!(two_count, 2);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    #[test]
    fn should_move_right_empty() {
        match board(2) {
            Ok(mut board) => {
                board.board = vec![0, 0, 0, 0];
                board.move_right();
                assert_eq!(board.board, vec![0, 0, 0, 0]);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
}
