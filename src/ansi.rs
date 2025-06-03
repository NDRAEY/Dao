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
