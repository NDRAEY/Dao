use std::io::Write;

use crate::{
    ansi,
    aux::Size,
    ui::{
        calculate_center_from, draw_box,
        input::{Key, get_key},
    },
};

#[derive(Debug, Clone, Copy, Default)]
pub enum MessageBoxButtons {
    #[default]
    Ok,
    // YesNo,
}

pub enum MessageBoxResult {
    Ok,
    // Yes,
    // No,
}

pub struct AlertMessage {
    pub message: String,
    pub buttons: MessageBoxButtons,
}

impl AlertMessage {
    pub fn new(message: String) -> Self {
        Self {
            message,
            buttons: MessageBoxButtons::default(),
        }
    }

    pub fn with_buttons(message: String, buttons: MessageBoxButtons) -> Self {
        Self { message, buttons }
    }

    pub fn show(&self, x: usize, y: usize) -> MessageBoxResult {
        let width = self.message.len() + 4; // `| ` + ` |`
        let height = 4;

        draw_box(x, y, width, height);

        ansi::move_cursor(y + 1, x + 2);
        print!("{}", self.message);

        match self.buttons {
            MessageBoxButtons::Ok => {
                ansi::move_cursor(y + 3, x + 2);
                ansi::inverse();
                print!("Ok");
                ansi::reset();
            }
        }

        std::io::stdout().lock().flush().unwrap();

        loop {
            let key = get_key();
            if let Key::Enter = key {
                return MessageBoxResult::Ok;
            }
        }
    }

    pub fn show_centered(&self, terminal_size: &Size) {
        let box_size = Size {
            columns: self.message.len() + 4,
            rows: 4,
        };
        let (x, y) = calculate_center_from(terminal_size, &box_size);
        self.show(x, y);
    }
}
