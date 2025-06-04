pub fn code(code: &str) {
    print!("\u{1b}{code}");
}

pub fn move_to_beginning() {
    code("[H");
}

pub fn move_cursor(ln: usize, column: usize) {
    code(&format!("{ln};{column}H"));
}

pub fn clean_screen() {
    code("[2J");
}

pub fn bold(code: u8) {
	code("[1m");
}

pub fn color(code: u8) {
	code("[{code}m");
}

pub fn reset() (
	code("[0m");
)
