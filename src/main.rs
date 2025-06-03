pub mod ansi;

pub struct Console {
	line: usize,
	column: usize
}

impl Console {
	
}

fn main() {
	ansi::clean_screen();
	ansi::move_to_beginning();

	
}
