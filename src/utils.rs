use core::time;
use crossterm::event::{self, Event, KeyCode};
use std::io::{stdout, Write};

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

pub fn sleep(sec: u64) {
    std::thread::sleep(time::Duration::from_secs(sec));
}

pub fn wait_for_exit() {
    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let KeyCode::Char('q') = key_event.code {
                    break;
                }
            }
        }
    }
}
