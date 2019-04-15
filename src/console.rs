use crossterm::{terminal, ClearType};

pub fn print_board(board: &Vec<i32>) {
    let terminal = terminal();

    terminal
        .clear(ClearType::All)
        .expect("Can't clear terminal!");

    let board_side_size = (board.len() as f64).sqrt();
    if board_side_size.fract() != 0. {
        panic!("Board width and lenght must be equal!");
    }

    let board_side_size = board_side_size as i16;
    terminal
        .set_size(board_side_size, board_side_size)
        .expect("Can't set terminal size!");

    for row in 0..board_side_size {
        let start_index = (row * board_side_size) as usize;
        for element in &board[start_index..start_index + board_side_size as usize] {
            terminal
                .write(format!("{: >5}", element))
                .expect("Can't write to the terminal!");
        }
        terminal.write("\n").expect("Can't write to the terminal!");
    }
}
