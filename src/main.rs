use std::{thread, time::Duration};

use mouse_rs::Mouse;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(60));
        move_mouse(100, 101);
        thread::sleep(Duration::from_secs(60));
        move_mouse(100, 100);
    }
}

fn move_mouse(x: i32, y: i32) {
    let mouse = Mouse::new();
    mouse.move_to(x, y).expect("Unable to move mouse");
}
