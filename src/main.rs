pub mod ansi;
pub mod termsize;

pub struct Console {
    line: usize,
    column: usize,
}

impl Console {}

fn main() {
    let size = termsize::get_terminal_size();

    ansi::clean_screen();
    ansi::move_to_beginning();

    for i in 0..size.columns {
    	print!("=");
    }
}
