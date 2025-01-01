use core::time;
use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};
use std::io::{stdout, Write};

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

pub fn sleep(sec: u64) {
    std::thread::sleep(time::Duration::from_secs(sec));
}

pub fn wait_for_exit() {
    Keyboard::A.bind(|_| {
        println!("A pressed, sending B");
        Keyboard::B.click();
    });
}
