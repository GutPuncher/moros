use crate::{kernel, print};
use alloc::string::String;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

pub fn color(name: &str) -> &str {
    match name {
        "Black"      => "\x1b[30m",
        "Red"        => "\x1b[31m",
        "Green"      => "\x1b[32m",
        "Brown"      => "\x1b[33m",
        "Blue"       => "\x1b[34m",
        "Magenta"    => "\x1b[35m",
        "Cyan"       => "\x1b[36m",
        "LightGray"  => "\x1b[37m",
        "DarkGray"   => "\x1b[90m",
        "LightRed"   => "\x1b[91m",
        "LightGreen" => "\x1b[92m",
        "Yellow"     => "\x1b[93m",
        "LightBlue"  => "\x1b[94m",
        "Pink"       => "\x1b[95m",
        "LightCyan"  => "\x1b[96m",
        "White"      => "\x1b[97m",
        "Reset"      => "\x1b[0m",
        _            => "",
    }
}

lazy_static! {
    pub static ref STDIN: Mutex<String> = Mutex::new(String::new());
    pub static ref ECHO: Mutex<bool> = Mutex::new(true);
    pub static ref RAW: Mutex<bool> = Mutex::new(false);
}

pub fn has_cursor() -> bool {
    cfg!(feature = "vga")
}

pub fn clear_row_after(x: usize) {
    if cfg!(feature = "vga") {
        kernel::vga::clear_row_after(x);
    } else {
        print!("\r"); // Move cursor to begining of line
        print!("\x1b[{}C", x); // Move cursor forward to position
        print!("\x1b[K"); // Clear line after position
    }
}

pub fn cursor_position() -> (usize, usize) {
    if cfg!(feature = "vga") {
        kernel::vga::cursor_position()
    } else {
        print!("\x1b[6n"); // Ask cursor position
        get_char(); // ESC
        get_char(); // [
        let mut x = String::new();
        let mut y = String::new();
        loop {
            let c = get_char();
            if c == ';' {
                break;
            } else {
                y.push(c);
            }
        }
        loop {
            let c = get_char();
            if c == 'R' {
                break;
            } else {
                x.push(c);
            }
        }
        (x.parse().unwrap_or(1), y.parse().unwrap_or(1))
    }
}

pub fn set_writer_position(x: usize, y: usize) {
    if cfg!(feature = "vga") {
        kernel::vga::set_writer_position(x, y);
    } else {
        print!("\x1b[{};{}H", y + 1, x + 1);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        if cfg!(feature="vga") {
            $crate::kernel::vga::print_fmt(format_args!($($arg)*));
        } else {
            $crate::kernel::serial::print_fmt(format_args!($($arg)*));
        }
    });
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        let uptime = $crate::kernel::clock::uptime();
        let csi_color = $crate::kernel::console::color("LightGreen");
        let csi_reset = $crate::kernel::console::color("Reset");
        if cfg!(feature="vga") {
            $crate::kernel::vga::print_fmt(format_args!("{}[{:.6}]{} ", csi_color, uptime, csi_reset));
            $crate::kernel::vga::print_fmt(format_args!($($arg)*));
        } else {
            $crate::kernel::serial::print_fmt(format_args!("{}[{:.6}]{} ", csi_color, uptime, csi_reset));
            $crate::kernel::serial::print_fmt(format_args!($($arg)*));
        }
    });
}

pub fn disable_echo() {
    let mut echo = ECHO.lock();
    *echo = false;
}

pub fn enable_echo() {
    let mut echo = ECHO.lock();
    *echo = true;
}

pub fn is_echo_enabled() -> bool {
    *ECHO.lock()
}

pub fn disable_raw() {
    let mut raw = RAW.lock();
    *raw = false;
}

pub fn enable_raw() {
    let mut raw = RAW.lock();
    *raw = true;
}

pub fn is_raw_enabled() -> bool {
    *RAW.lock()
}

pub fn key_handle(key: char) {
    let mut stdin = STDIN.lock();

    if key == '\x08' && !is_raw_enabled() {
        // Avoid printing more backspaces than chars inserted into STDIN.
        // Also, the VGA driver support only ASCII so unicode chars will
        // be displayed with one square for each codepoint.
        if stdin.len() > 0 {
            let n = stdin.pop().unwrap().len_utf8();
            if is_echo_enabled() {
                for _ in 0..n {
                    print!("\x08");
                }
            }
        }
    } else {
        // TODO: Replace non-ascii chars by ascii square symbol to keep length
        // at 1 instead of being variable?
        stdin.push(key);
        if is_echo_enabled() {
            print!("{}", key);
        }
    }
}

pub fn get_char() -> char {
    kernel::console::disable_echo();
    kernel::console::enable_raw();
    loop {
        kernel::time::halt();
        let res = interrupts::without_interrupts(|| {
            let mut stdin = STDIN.lock();
            match stdin.chars().next_back() {
                Some(c) => {
                    stdin.clear();
                    Some(c)
                },
                _ => {
                    None
                }
            }
        });
        if let Some(c) = res {
            kernel::console::enable_echo();
            kernel::console::disable_raw();
            return c;
        }
    }
}

pub fn get_line() -> String {
    loop {
        kernel::time::halt();
        let res = interrupts::without_interrupts(|| {
            let mut stdin = STDIN.lock();
            match stdin.chars().next_back() {
                Some('\n') => {
                    let line = stdin.clone();
                    stdin.clear();
                    Some(line)
                }
                _ => {
                    None
                }
            }
        });
        if let Some(line) = res {
            return line;
        }
    }
}
