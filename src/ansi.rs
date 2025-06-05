pub fn code(code: &str) {
    print!("\u{1b}{code}");
}

pub fn move_to_beginning() {
    code("[H");
}

pub fn move_cursor(ln: usize, column: usize) {
    code(&format!("[{ln};{column}H"));
}

pub fn clean_screen() {
    code("[2J");
}

pub fn bold() {
    code("[1m");
}

pub fn color(color_code: u8) {
    code(&format!("[{color_code}m"));
}

pub fn inverse() {
    code("[7m");
}

pub fn reset() {
    code("[0m");
}

pub fn clean_till_end() {
    code("[K");
}
