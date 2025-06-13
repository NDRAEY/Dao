// Since NocturneOS is known to nobody, I allow unexpected cfgs.
#![allow(unexpected_cfgs)]

use crate::aux::Size;
#[cfg(target_os = "nocturne")]
use std::os::nocturne::terminal;

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

#[cfg(target_os = "nocturne")]
pub fn get_terminal_size_nocturne() -> Size {
    let ts = terminal::terminal_size();

    Size {
        rows: s.0,
        columns: s.1,
    }
}

pub fn get_terminal_size() -> Size {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        get_terminal_size_linux()
    }
    #[cfg(target_os = "nocturne")]
    {
        get_terminal_size_nocturne()
    }
}
