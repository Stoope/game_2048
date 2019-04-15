use crossterm::{input, InputEvent, KeyEvent, RawScreen};

fn check_quit(key_event: InputEvent) -> bool {
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
fn process_board(key_event: InputEvent, board: &Vec<i32>) -> Option<Vec<i32>> {
    match key_event {
        InputEvent::Keyboard(k) => match k {
            KeyEvent::Up => None,
            _ => None,
        },
        _ => None,
    }
}

pub fn start_game(board_size: i16) {
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        let input = input();

        let mut sync_stdin = input.read_sync();

        loop {
            let event = sync_stdin.next();

            if let Some(key_event) = event {
                if check_quit(key_event) {
                    break;
                } else {
                }
            }
        }
    }
}
