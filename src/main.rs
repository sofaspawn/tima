use ffi::IsKeyDown;
use raylib::prelude::*;
use chrono::prelude::*;

const WIDTH:i32 = 2000;
const HEIGHT:i32 = 1000;

fn formattime(anchor: DateTime<Local>)->String{
    let elapsed = Local::now() - anchor;
    let ehours = elapsed.num_hours();
    let eminutes = elapsed.num_minutes();
    let eseconds = elapsed.num_seconds();

    let ohours = if ehours<10{format!("0{}", ehours)} else {ehours.to_string()};
    let ominutes = if eminutes<10{format!("0{}", eminutes)} else {eminutes.to_string()};
    let oseconds = if eseconds<10{format!("0{}", eseconds)} else {eseconds.to_string()};

    format!("{}:{}:{}", ohours, ominutes, oseconds)
}

fn main() {
    //ffi::SetConfigFlags(ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE);
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("tima")
        .build();

    let font = 200;
    let init = Local::now();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let thms = formattime(init);

        unsafe{
            if IsKeyDown(KeyboardKey::KEY_Q as i32){
                break;
            }
        }
        let twid = rl.measure_text(thms.as_str(), font);
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text(thms.as_str(), WIDTH/2 - twid/2, HEIGHT/2, font, Color::WHITE);
    }
}
