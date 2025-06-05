use crate::aux::Size;

pub fn get_terminal_size() -> Size {
    unsafe {
        let mut value = core::mem::zeroed::<libc::winsize>();
        libc::ioctl(
            libc::STDOUT_FILENO,
            libc::TIOCGWINSZ,
            (&mut value as *mut libc::winsize).addr(),
        );

        Size {
            rows: value.ws_row as _,
            columns: value.ws_col as _,
        }
    }
}
