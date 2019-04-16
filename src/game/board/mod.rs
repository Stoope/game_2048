use crossterm::{InputEvent, KeyEvent};
use rand::Rng;
use std::io::{Error, ErrorKind};
mod console;

pub struct Board {
    board: Vec<i32>,
}

impl Board {
    pub fn new(board_size: i16) -> Result<Board, Error> {
        match board_size > 1 {
            true => {
                let mut board: Vec<i32> = vec![0; board_size.pow(2) as usize];

                let mut rng = rand::thread_rng();
                board[rng.gen_range(0, board_size as usize / 2)] = 2;
                board[rng.gen_range(board_size as usize / 2, board_size as usize)] = 2;

                Ok(Board { board })
            }
            false => Err(Error::new(
                ErrorKind::InvalidInput,
                "Board size must be greater then 1!",
            )),
        }
    }

    pub fn perform_action(&mut self, key_event: &InputEvent) {
        match key_event {
            InputEvent::Keyboard(k) => match k {
                KeyEvent::Char(c) => match c {
                    'd' => {
                        self.move_right();
                    }
                    'a' => {
                        self.move_left();
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        console::print_board(&self.board).expect("Error while processing board!");
    }

    fn get_board_size(&self) -> usize {
        (self.board.len() as f64).sqrt() as usize
    }

    fn move_right(&mut self) {
        let mut new_board = vec![];
        let board_size = self.get_board_size();
        for row in 0..board_size {
            let start_index = (row * board_size) as usize;
            let new_row = &Vec::from(&self.board[start_index..start_index + board_size as usize]);
            new_board.append(&mut fold_to_end(new_row));
        }
        self.board = new_board;
    }

    fn move_left(&mut self) {
        self.board = rotate_board_left_90_deg(&self.board);
        self.board = rotate_board_left_90_deg(&self.board);
        self.move_right();
        self.board = rotate_board_left_90_deg(&self.board);
        self.board = rotate_board_left_90_deg(&self.board);
    }
}

fn move_zeros_to_start(board_row: &Vec<i32>) -> Vec<i32> {
    let mut board_row_without_zeros: Vec<i32> = board_row
        .iter()
        .filter(|&item| *item != 0)
        .rev()
        .cloned()
        .collect();
    board_row_without_zeros.resize(board_row.len(), 0);
    let board_row_without_zeros = board_row_without_zeros.iter().rev();
    vec![0; board_row.len()]
        .iter()
        .zip(board_row_without_zeros)
        .map(|(&a, &b)| if b == 0 { a } else { b })
        .collect()
}

fn rotate_board_left_90_deg(board: &Vec<i32>) -> Vec<i32> {
    let board_size = (board.len() as f64).sqrt() as usize;
    let mut new_board: Vec<i32> = vec![];
    for column in (0..board_size).rev() {
        for row in 0..board_size {
            let index = ((row * board_size) + column) as usize;
            new_board.push(board[index]);
        }
    }
    new_board
}

fn fold_to_end(board_row: &Vec<i32>) -> Vec<i32> {
    let mut new_board_row = board_row.clone();
    new_board_row = move_zeros_to_start(&mut new_board_row);
    for x in (1..new_board_row.len()).rev() {
        match new_board_row[x.checked_sub(1).unwrap_or(0)] == new_board_row[x] {
            true => {
                new_board_row[x] *= 2;
                new_board_row[x.checked_sub(1).unwrap_or(0)] = 0;
            }
            false => {}
        }
    }
    new_board_row = move_zeros_to_start(&mut new_board_row);
    new_board_row
}

pub fn board(board_size: i16) -> Result<Board, Error> {
    Board::new(board_size)
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn should_rotate_board_left_90_deg_case_1() {
        assert_eq!(
            rotate_board_left_90_deg(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            vec![3, 6, 9, 2, 5, 8, 1, 4, 7]
        );
    }

    #[test]
    fn should_rotate_board_left_90_deg_case_2() {
        assert_eq!(
            rotate_board_left_90_deg(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            vec![4, 8, 12, 16, 3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13]
        );
    }

    #[test]
    fn should_move_zeros_to_start_case_1() {
        assert_eq!(move_zeros_to_start(&vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn should_move_zeros_to_start_case_2() {
        assert_eq!(move_zeros_to_start(&vec![2, 0, 0, 0]), vec![0, 0, 0, 2]);
    }

    #[test]
    fn should_move_zeros_to_start_case_3() {
        assert_eq!(move_zeros_to_start(&vec![2, 0, 4, 0]), vec![0, 0, 2, 4]);
    }

    #[test]
    fn should_move_zeros_to_start_case_4() {
        assert_eq!(move_zeros_to_start(&vec![0, 4, 0, 4]), vec![0, 0, 4, 4]);
    }

    #[test]
    fn should_fold_to_end_case_1() {
        assert_eq!(fold_to_end(&vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn should_fold_to_end_case_2() {
        assert_eq!(fold_to_end(&vec![4, 4, 2, 0]), vec![0, 0, 8, 2]);
    }

    #[test]
    fn should_fold_to_end_case_3() {
        assert_eq!(fold_to_end(&vec![2, 2, 2, 2]), vec![0, 0, 4, 4]);
    }

    #[test]
    fn should_fold_to_end_case_4() {
        assert_eq!(fold_to_end(&vec![2, 0, 0, 0]), vec![0, 0, 0, 2]);
    }

    #[test]
    fn should_fold_to_end_case_5() {
        assert_eq!(fold_to_end(&vec![2, 4]), vec![2, 4]);
    }

    #[test]
    fn should_return_error_with_board_size_0() -> Result<(), ()> {
        match board(0) {
            Ok(_) => Err(()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn should_get_board_size() {
        let board_size = 6;

        match board(board_size) {
            Ok(board) => {
                assert_eq!(board.get_board_size(), board_size as usize);
            }
            Err(e) => {
                panic!(e);
            }
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
    fn should_move_right_case_1() {
        match board(2) {
            Ok(mut board) => {
                board.board = vec![2, 4, 0, 8];
                board.move_right();
                assert_eq!(board.board, vec![2, 4, 0, 8]);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    #[test]
    fn should_move_right_case_2() {
        match board(2) {
            Ok(mut board) => {
                board.board = vec![4, 4, 4, 0];
                board.move_right();
                assert_eq!(board.board, vec![0, 8, 0, 4]);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    #[test]
    fn should_move_right_case_3() {
        match board(2) {
            Ok(mut board) => {
                board.board = vec![2, 2, 2, 2, 0, 2, 0, 2, 0];
                board.move_right();
                assert_eq!(board.board, vec![0, 2, 4, 0, 0, 4, 0, 0, 2]);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    #[test]
    fn should_move_left_case_1() {
        match board(2) {
            Ok(mut board) => {
                board.board = vec![2, 4, 0, 8];
                board.move_left();
                assert_eq!(board.board, vec![2, 4, 8, 0]);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    #[test]
    fn should_move_left_case_2() {
        match board(2) {
            Ok(mut board) => {
                board.board = vec![4, 4, 4, 0];
                board.move_left();
                assert_eq!(board.board, vec![8, 0, 4, 0]);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    #[test]
    fn should_move_left_case_3() {
        match board(2) {
            Ok(mut board) => {
                board.board = vec![2, 2, 2, 2, 0, 2, 0, 2, 0];
                board.move_left();
                assert_eq!(board.board, vec![4, 2, 0, 4, 0, 0, 2, 0, 0]);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
}
