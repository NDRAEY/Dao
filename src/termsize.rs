use crate::aux::Size;

#[cfg(any(target_os = "linux", target_os = "android"))]
fn get_terminal_size_linux() -> Size {
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

pub fn get_terminal_size() -> Size {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        return get_terminal_size_linux();
    }

    Size {
        rows: 0,
        columns: 0,
    }
}
