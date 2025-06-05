use libc::ECHO;
use termios::{self, ICANON, TCSANOW, Termios, tcsetattr};

pub struct TerminalContext {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    termios: Termios,
}

fn linux_setup() -> Termios {
    let termios = termios::Termios::from_fd(libc::STDIN_FILENO).unwrap();
    let mut new_termios = termios;
    new_termios.c_lflag &= !(ICANON | ECHO);

    tcsetattr(libc::STDIN_FILENO, TCSANOW, &mut new_termios).unwrap();

    termios
}

fn linux_restore(termios: Termios) {
    tcsetattr(libc::STDIN_FILENO, TCSANOW, &termios).unwrap();
}

pub fn setup() -> TerminalContext {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let termios = linux_setup();

        TerminalContext { termios }
    }
}

pub fn restore(context: &TerminalContext) {
        #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        linux_restore(context.termios);
    }
}
