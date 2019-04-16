use crossterm::{input, InputEvent, KeyEvent, RawScreen};
mod board;

fn check_quit(key_event: &InputEvent) -> bool {
    match key_event {
        InputEvent::Keyboard(k) => match k {
            KeyEvent::Char(c) => match c {
                'q' => true,
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

pub fn start_game(board_size: i16) {
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        let input = input();
        match board::board(board_size) {
            Ok(board) => {
                let mut sync_stdin = input.read_sync();

                loop {
                    let event = sync_stdin.next();

                    if let Some(key_event) = event {
                        if check_quit(&key_event) {
                            break;
                        } else {
                            board.perform_action(&key_event);
                        }
                    }
                }
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
}
