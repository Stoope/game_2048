use crossterm::{InputEvent, KeyEvent};
mod console;

pub struct Board {
    board: Vec<i32>,
}

impl Board {
    pub fn new(board_size: i16) -> Board {
        let board: Vec<i32> = vec![0; board_size.pow(2) as usize];

        Board { board }
    }

    pub fn perform_action(&self, key_event: &InputEvent) {
        match key_event {
            InputEvent::Keyboard(k) => match k {
                KeyEvent::Up => {
                    console::print_board(&self.board).expect("Error while processing board!");
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn board(board_size: i16) -> Board {
    Board::new(board_size)
}
