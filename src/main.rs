#![no_std]
#![no_main]

mod vga_buffer;
mod serial;

use pc_keyboard::DecodedKey;
use csci320_match3::HandlerTable;

fn start() {
    println!("Hello, world!");
}

fn tick() {
    print!(".");
}

fn key(key: DecodedKey) {
    match key {
        DecodedKey::Unicode(character) => print!("{}", character),
        DecodedKey::RawKey(key) => print!("{:?}", key),
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .startup(start)
        .start()
}