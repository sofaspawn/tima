use ffi::IsKeyDown;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("tima")
        .build();

    while !rl.window_should_close() {
        IsKeyDown(KeyboardKey::KEY_Q as i32);
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
