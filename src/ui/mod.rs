use crate::{ansi, aux::Size};

pub mod alertbox;
pub mod input;

pub fn calculate_center_from(terminal_size: &Size, box_size: &Size) -> (usize, usize) {
    let x = (terminal_size.columns - box_size.columns) / 2;
    let y = (terminal_size.rows - box_size.rows) / 2;
    (x, y)
}

pub fn draw_box(x: usize, y: usize, width: usize, height: usize) {
    ansi::move_cursor(y, x);
    print!("┌");
    for _ in 0..width {
        print!("─");
    }
    print!("┐");

    for iy in 0..height {
        ansi::move_cursor(y + 1 + iy, x);
        print!("│");
        for _ in 0..width {
            print!(" ");
        }
        print!("│");
    }

    ansi::move_cursor(y + height, x);
    print!("└");

    for _ in 0..width {
        print!("─");
    }
    print!("┘");
}
