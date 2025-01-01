use core::time;

pub fn clear() {
    print!("{}[2J", 27 as char);
}

pub fn sleep(time: u64) {
    std::thread::sleep(time::Duration::from_secs(time));
}
