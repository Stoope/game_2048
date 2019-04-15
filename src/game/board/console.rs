use crossterm::{terminal, ClearType};
use std::io;

pub fn print_board(board: &Vec<i32>) -> io::Result<()> {
    let terminal = terminal();

    terminal.clear(ClearType::All)?;

    let board_side_size = (board.len() as f64).sqrt();
    if board_side_size.fract() != 0. {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Board width and lenght must be equal!",
        ));
    }

    let board_side_size = board_side_size as i16;
    terminal.set_size(board_side_size, board_side_size)?;

    for row in 0..board_side_size {
        let start_index = (row * board_side_size) as usize;
        for element in &board[start_index..start_index + board_side_size as usize] {
            terminal.write(format!("{: >5}", element))?;
        }
        terminal.write("\r\n")?;
    }

    Ok(())
}
