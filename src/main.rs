use std::io::{Read, Write};

use text_editor_foundation::VirtualEditor;

pub mod ansi;
pub mod aux;
pub mod keys;
pub mod terminal;
pub mod termsize;
pub mod ui;

// pub struct Console {
//     line: usize,
//     column: usize,
// }

// impl Console {}

struct Editor {
    terminal_size: aux::Size,
    editor: VirtualEditor,

    terminal_context: terminal::TerminalContext,
    filename: Option<String>,
}

impl Editor {
    pub fn new() -> Self {
        let terminal_size = termsize::get_terminal_size();

        Self {
            terminal_size,
            editor: VirtualEditor::new(),
            terminal_context: terminal::setup(),
            filename: None,
        }
    }

    fn draw_header(&self) {
        let header_string = format!("- Dao v{} -", env!("CARGO_PKG_VERSION"));

        print!("{}", header_string);

        for _ in 0..(self.terminal_size.columns - header_string.len()) {
            print!("-");
        }

        println!();
    }

    fn text_window_height(&self) -> usize {
        self.terminal_size.rows - 1
    }

    fn scroll_factor(&self) -> usize {
        let line_renders_count = self.text_window_height();
        let current_line = self.editor.cursor().y;

        current_line.saturating_sub(line_renders_count - (line_renders_count / 4))
    }

    fn draw_body(&self) {
        let line_renders_count = self.text_window_height();
        let line_number_padding = core::cmp::max(self.editor.line_count(), line_renders_count)
            .to_string()
            .len();
        let lines = self.editor.lines();
        let len = lines.len();

        let scroll_factor = self.scroll_factor();

        for i in 1..line_renders_count {
            ansi::move_cursor(i + 1, 0);

            let lineidx = i - 1 + scroll_factor;
            let line = if lineidx < len { lines[lineidx] } else { "" };
            print!("{:>line_number_padding$} | {}", lineidx + 1, line);

            ansi::clean_till_end();
        }

        println!();
    }

    fn place_cursor(&self) {
        let pos = self.editor.cursor();

        let line_renders_count = self.text_window_height();
        let line_number_padding = core::cmp::max(self.editor.line_count(), line_renders_count)
            .to_string()
            .len();

        let x = pos.x + line_number_padding + 4; // 4 is the padding of the line number and the ` | `
        let y = pos.y + 2;

        ansi::move_cursor(
            y.clamp(2, line_renders_count - (line_renders_count / 4) + 2),
            x,
        );
    }

    fn draw_footer(&self) {
        let footer_string = format!(
            "-- {} --- L{} C{} --- {}x{} ---",
            self.filename
                .as_ref()
                .unwrap_or(&"<No Filename>".to_string()),
            self.editor.cursor().y + 1,
            self.editor.cursor().x + 1,
            self.terminal_size.columns,
            self.terminal_size.rows
        );

        print!("{}", footer_string);

        for _ in 0..(self.terminal_size.columns - footer_string.len()) {
            print!("-");
        }
    }

    fn flush_screen(&self) {
        std::io::stdout().flush().unwrap();
    }

    fn process_key(&mut self, key: char, additional: &[u8]) -> bool {
        match key {
            keys::CTRL_Q => {
                ansi::clean_screen();
                ansi::move_to_beginning();

                true
            }
            keys::ENTER => {
                self.editor.insert_str_move("\n");

                false
            }
            keys::BACKSPACE => {
                self.editor.delete_char_left_nocheck();

                if self.editor.cursor().x == 0 {
                    self.editor.move_up();
                    self.editor.move_to_line_end();
                } else {
                    self.editor.move_left();
                }

                false
            }
            '\u{1b}' => {
                if additional == keys::ADDITIONAL_ARROW_UP {
                    self.editor.move_up();
                    if self.editor.cursor().x > self.editor.get_line_at_cursor().len() {
                        self.editor.move_to_line_end();
                    }
                } else if additional == keys::ADDITIONAL_ARROW_DOWN {
                    self.editor.move_down();
                    if self.editor.cursor().x > self.editor.get_line_at_cursor().len() {
                        self.editor.move_to_line_end();
                    }
                } else if additional == keys::ADDITIONAL_ARROW_RIGHT {
                    self.editor.move_right();
                } else if additional == keys::ADDITIONAL_ARROW_LEFT {
                    self.editor.move_left();
                } else if additional == keys::ADDITIONAL_ARROW_DELETE {
                    self.editor.delete_char_right_nocheck();
                } else if additional == keys::ADDITIONAL_ARROW_HOME {
                    self.editor.move_to_line_begin();
                } else if additional == keys::ADDITIONAL_ARROW_END {
                    self.editor.move_to_line_end();
                }

                false
            }
            key if key.is_ascii_control() => false,
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
    let filename = std::env::args().nth(1);

    let mut editor = Editor::new();

    if let Some(filename) = filename {
        let file = std::fs::read_to_string(&filename).unwrap();

        editor.editor.insert_str(&file);
        editor.filename = Some(filename);
    }

    ansi::clean_screen();

    editor.render();

    //input_test();

    loop {
        let mut buf = [0; 4];
        let rdc = std::io::stdin().read(&mut buf).unwrap();

        let mut chars = str::from_utf8(&buf[0..rdc])
            .unwrap()
            .trim_end_matches('\0')
            .chars();

        let key = chars.next().unwrap();

        let additionals = &buf[key.len_utf8()..];

        let is_exit = editor.process_key(key, additionals);

        if is_exit {
            break;
        } else {
            editor.render();
        }
    }

    println!("{}", editor.editor.text());
}

/*
fn input_test() {
    loop {
        let mut buf = [0; 4];
         let rdc = std::io::stdin().read(&mut buf).unwrap();

         let mut chars = str::from_utf8(&buf[0..rdc])
             .unwrap()
             .trim_end_matches('\0')
             .chars();

         let key = chars.next().unwrap();

         let additionals = &buf[key.len_utf8()..];

         dbg!(&key, &additionals);
     }
 }
*/
