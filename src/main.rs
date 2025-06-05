use libc::LINUX_REBOOT_CMD_CAD_ON;
use std::io::{Read, Write};
use text_editor_foundation::VirtualEditor;

pub mod ansi;
pub mod aux;
pub mod keys;
pub mod terminal;
pub mod termsize;

// pub struct Console {
//     line: usize,
//     column: usize,
// }

// impl Console {}

struct Editor {
    terminal_size: aux::Size,
    editor: VirtualEditor,

    terminal_context: terminal::TerminalContext,
}

impl Editor {
    pub fn new() -> Self {
        let terminal_size = termsize::get_terminal_size();

        Self {
            terminal_size,
            editor: VirtualEditor::new(),
            terminal_context: terminal::setup(),
        }
    }

    fn draw_header(&self) {
        let header_string = format!("- Dao v.{} -", env!("CARGO_PKG_VERSION"));

        print!("{}", header_string);

        for i in 0..(self.terminal_size.columns - header_string.len()) {
            print!("-");
        }

        println!();
    }

    fn draw_body(&self) {
        let line_renders_count = self.terminal_size.rows - 2;
        let line_number_padding = core::cmp::max(self.editor.line_count(), line_renders_count)
            .to_string()
            .len();
        let lines = self.editor.lines();
        let len = lines.len();

        for i in 1..line_renders_count {
            let lineidx = i - 1;

            let line = if lineidx < len { lines[lineidx] } else { "" };

            ansi::move_cursor(i + 1, 0);

            print!("{:>line_number_padding$} | {}", i, line);

            ansi::clean_till_end();
        }

        println!();
    }

    fn place_cursor(&self) {
        let pos = self.editor.cursor();

        let line_renders_count = self.terminal_size.rows - 2;
        let line_number_padding = core::cmp::max(self.editor.line_count(), line_renders_count)
            .to_string()
            .len();

        let x = pos.x + line_number_padding + 4; // 4 is the padding of the line number and the ` | `
        let y = pos.y + 1;

        ansi::move_cursor(y + 1, x);
    }

    fn draw_footer(&self) {
        let footer_string = format!(
            "- {}x{} -",
            self.terminal_size.columns, self.terminal_size.rows
        );

        print!("{}", footer_string);

        for i in 0..(self.terminal_size.columns - footer_string.len()) {
            print!("-");
        }
    }

    fn flush_screen(&self) {
        std::io::stdout().flush().unwrap();
    }

    fn process_key(&mut self, key: char, additional: &[u8]) -> bool {
        match key {
            keys::CTRL_Q => {
                // Ctrl + Q
                ansi::clean_screen();
                ansi::move_to_beginning();

                return true;
            }
            keys::ENTER => {
                self.editor.insert_str_move("\n");

                false
            }
            keys::BACKSPACE => {
                self.editor.delete_char_left();

                if self.editor.cursor().x == 0 {
                    self.editor.move_up();
                    self.editor.move_to_line_end();
                } else {
                    self.editor.move_left();
                }

                false
            }
            _ => {
                self.editor.insert_char_move(key);

                false
            }
        }
    }

    fn render(&self) {
        ansi::move_to_beginning();

        self.draw_header();
        self.draw_body();
        self.draw_footer();
        self.place_cursor();
        self.flush_screen();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::restore(&self.terminal_context);
    }
}

fn main() {
    let mut editor = Editor::new();

    ansi::clean_screen();

    // editor.render();

    // Read key by key and update the editor
    loop {
        let mut buf = [0; 4];
        let rdc = std::io::stdin().read(&mut buf).unwrap();

        let mut chars = str::from_utf8(&buf[0..rdc])
            .unwrap()
            .trim_end_matches('\0')
            .chars();

        let key = chars
            .nth(0)
            .unwrap();

        let additionals = &buf[key.len_utf8()..];

        dbg!(&key);

        let is_exit = editor.process_key(key, additionals);

        if is_exit {
            break;
        } else {
            // editor.render();
        }
    }

    println!("{}", editor.editor.text());
}
