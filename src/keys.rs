pub const CTRL_Q: char = '\x11';
pub const CTRL_S: char = '\x13';
pub const ENTER: char = '\n';
//#[cfg(target_os = "nocturne")]
//pub const BACKSPACE: char = '\x0e';
//#[cfg(not(target_os = "nocturne"))]
pub const BACKSPACE: char = '\x7f';
pub const ADDITIONAL_ARROW_UP: &[u8] = &[0x5b, 0x41, 0x00];
pub const ADDITIONAL_ARROW_DOWN: &[u8] = &[0x5b, 0x42, 0x00];
pub const ADDITIONAL_ARROW_RIGHT: &[u8] = &[0x5b, 0x43, 0x00];
pub const ADDITIONAL_ARROW_LEFT: &[u8] = &[0x5b, 0x44, 0x00];
pub const ADDITIONAL_ARROW_DELETE: &[u8] = &[0x5b, 0x33, 0x7e];

pub const ADDITIONAL_ARROW_HOME: &[u8] = &[0x5b, 0x48, 0x00];
pub const ADDITIONAL_ARROW_END: &[u8] = &[0x5b, 0x46, 0x00];
