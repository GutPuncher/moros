#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use moros::{print, user, kernel};

entry_point!(main);

fn main(_boot_info: &'static BootInfo) -> ! {
    moros::init();
    include_file("/cfg/boot.sh", include_str!("../dsk/cfg/boot.sh"));
    include_file("/cfg/banner.txt", include_str!("../dsk/cfg/banner.txt"));
    include_file("/cfg/passwords.csv", include_str!("../dsk/cfg/passwords.csv"));
    loop {
        user::shell::main(&["shell", "/cfg/boot.sh"]);
    }
}

fn include_file(pathname: &str, contents: &str) {
    if let Some(mut file) = kernel::fs::File::create(pathname) {
        file.write(contents);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}\n", info);
    loop { kernel::sleep::sleep(10.0) }
}
