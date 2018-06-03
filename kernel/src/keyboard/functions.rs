use ::spin::Mutex;
use super::drivers::PS2Driver;

pub static SYSTEM_KEYBOARD: Mutex<PS2Driver> = Mutex::new(PS2Driver::new());