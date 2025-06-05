use std::io::Read;

use crate::keys;

pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    Delete,
    Backspace,
    Enter,
    Escape,
    Tab,
    None
}

pub fn get_key() -> Key {
    let mut buf = [0; 4];
    let rdc = std::io::stdin().read(&mut buf).unwrap();

    let mut chars = str::from_utf8(&buf[0..rdc])
        .unwrap()
        .trim_end_matches('\0')
        .chars();

    let key = chars.nth(0).unwrap();

    let additional = &buf[key.len_utf8()..];

    match key {
        '\u{1b}' => {
            if additional == keys::ADDITIONAL_ARROW_UP {
                Key::Up
            } else if additional == keys::ADDITIONAL_ARROW_DOWN {
                Key::Down
            } else if additional == keys::ADDITIONAL_ARROW_RIGHT {
                Key::Right
            } else if additional == keys::ADDITIONAL_ARROW_LEFT {
                Key::Left
            } else if additional == keys::ADDITIONAL_ARROW_DELETE {
                Key::Delete
            } else if additional == keys::ADDITIONAL_ARROW_HOME {
                Key::Home
            } else if additional == keys::ADDITIONAL_ARROW_END {
                Key::End
            } else {
                Key::Escape
            }
        }
        '\n' => Key::Enter,
        '\t' => Key::Tab,
        _ => Key::None,
    }
}
