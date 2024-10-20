use chrono::prelude::*;
use ffi::{IsKeyDown, IsKeyPressed};
use rand::Rng;
use raylib::prelude::*;
use std::env;

const WIDTH: i32 = 2000;
const HEIGHT: i32 = 1000;
const FPS: u32 = 60;

fn timemvmnt(counter: i32, pos: &mut ffi::Vector2) {
    //agressive shaking
    if counter % 1 == 0 {
        // counter for customization
        pos.x += rand::thread_rng().gen_range(-20..21) as f32;
        pos.y += rand::thread_rng().gen_range(-20..21) as f32;
    }
}

fn handle_args(args: &Vec<String>) -> Option<String> {
    if args.len() > 2 {
        return None;
    } else if args.len() == 2 {
        return Some(args[1].clone());
    } else {
        return Some(args[0].clone());
    }
}

fn secsformat(timeinframe: i32) -> String {
    let secs = timeinframe / (FPS as i32);
    let min = secs / 60;
    let hour = secs / 3600;

    let osecs = if secs - 60 * (secs/60) < 10 {
        format!("0{}", secs - 60 * (secs/60))
    } else if secs>=60{
        format!("{}", secs - 60 * (secs/60))
    } else {
        format!("{secs}")
    };

    let omin = if min - 60 * (min/60) < 10 {
        format!("0{}", min - 60 * (min/60))
    } else if min>=60{
        format!("{}", min - 60 * (min/60))
    } else {
        format!("{min}")
    };

    let ohour = if hour - 60 * (hour/60) < 10 {
        format!("0{}", hour - 60 * (hour/60))
    } else if hour>=60{
        format!("{}", hour - 60 * (hour/60))
    } else {
        format!("{hour}")
    };

    format!("{ohour}:{omin}:{osecs}")
}

fn stopwatch(
    mut rl: RaylibHandle,
    thread: RaylibThread,
    mut camera: ffi::Camera2D,
    mut scale: f64,
    font: Font,
    font_size: i32,
    mut cwid: i32,
    mut chit: i32,
) {
    let mut vib_counter = 0;
    let mut color = Color::WHITE;

    let mut timeinframe = 0;

    while !rl.window_should_close() {
        let thms = secsformat(timeinframe);
        timeinframe += 1;

        unsafe {
            if IsKeyDown(KeyboardKey::KEY_Q as i32) {
                break;
            }
            if IsKeyDown(KeyboardKey::KEY_EQUAL as i32) {
                scale += 2.0;
            }
            if IsKeyDown(KeyboardKey::KEY_MINUS as i32) {
                if scale > 5.0 {
                    scale -= 2.0;
                }
            }
            if IsKeyDown(KeyboardKey::KEY_ZERO as i32) {
                scale = 400.0;
            }
            if IsKeyPressed(KeyboardKey::KEY_SPACE as i32) {
                if color == Color::WHITE {
                    color = Color::PINK;
                } else if color == Color::PINK {
                    color = Color::WHITE;
                }
            }
        }

        let twid = rl.measure_text(thms.as_str(), (font_size as f64 * scale) as i32);
        let thi = (font_size as f64 * scale) as i32;

        cwid = rl.get_screen_width();
        chit = rl.get_screen_height();

        let mut pos = ffi::Vector2 {
            x: ((cwid - twid) / 2) as f32,
            y: ((chit - thi) / 2) as f32,
        };

        timemvmnt(vib_counter, &mut pos);

        camera.zoom = scale as f32;
        camera.offset = pos;
        camera.target = pos;

        vib_counter += 1;

        unsafe {
            ffi::BeginMode2D(camera);
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text_ex(
            &font,
            thms.as_str(),
            pos,
            font_size as f32 * scale as f32,
            10.0,
            color,
        );
        unsafe {
            ffi::EndMode2D();
        }
    }
}

fn timer(
    mut rl: RaylibHandle,
    thread: RaylibThread,
    mut camera: ffi::Camera2D,
    mut scale: f64,
    font: Font,
    font_size: i32,
    mut cwid: i32,
    mut chit: i32,
) {
    let mut vib_counter = 0;
    let mut color = Color::WHITE;

    let mut timeinframe = 0;

    while !rl.window_should_close() {
        let thms = secsformat(timeinframe);
        timeinframe += 1;

        unsafe {
            if IsKeyDown(KeyboardKey::KEY_Q as i32) {
                break;
            }
            if IsKeyDown(KeyboardKey::KEY_EQUAL as i32) {
                scale += 2.0;
            }
            if IsKeyDown(KeyboardKey::KEY_MINUS as i32) {
                if scale > 5.0 {
                    scale -= 2.0;
                }
            }
            if IsKeyDown(KeyboardKey::KEY_ZERO as i32) {
                scale = 400.0;
            }
            if IsKeyPressed(KeyboardKey::KEY_SPACE as i32) {
                if color == Color::WHITE {
                    color = Color::PINK;
                } else if color == Color::PINK {
                    color = Color::WHITE;
                }
            }
        }

        let twid = rl.measure_text(thms.as_str(), (font_size as f64 * scale) as i32);
        let thi = (font_size as f64 * scale) as i32;

        cwid = rl.get_screen_width();
        chit = rl.get_screen_height();

        let mut pos = ffi::Vector2 {
            x: ((cwid - twid) / 2) as f32,
            y: ((chit - thi) / 2) as f32,
        };

        timemvmnt(vib_counter, &mut pos);

        camera.zoom = scale as f32;
        camera.offset = pos;
        camera.target = pos;

        vib_counter += 1;

        unsafe {
            ffi::BeginMode2D(camera);
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text_ex(
            &font,
            thms.as_str(),
            pos,
            font_size as f32 * scale as f32,
            10.0,
            color,
        );
        unsafe {
            ffi::EndMode2D();
        }
    }
}

fn clock(
    mut rl: RaylibHandle,
    thread: RaylibThread,
    mut camera: ffi::Camera2D,
    mut scale: f64,
    font: Font,
    font_size: i32,
    mut cwid: i32,
    mut chit: i32,
) {
    let mut vib_counter = 0;
    let mut color = Color::WHITE;
    while !rl.window_should_close() {
        let local = Local::now();
        //make the time format consistent i.e. 1 second should be shown as 01
        let thms = format!("{}:{}:{}", local.hour(), local.minute(), local.second());

        unsafe {
            if IsKeyDown(KeyboardKey::KEY_Q as i32) {
                break;
            }
            if IsKeyDown(KeyboardKey::KEY_EQUAL as i32) {
                scale += 2.0;
            }
            if IsKeyDown(KeyboardKey::KEY_MINUS as i32) {
                if scale > 5.0 {
                    scale -= 2.0;
                }
            }
            if IsKeyDown(KeyboardKey::KEY_ZERO as i32) {
                scale = 400.0;
            }
            if IsKeyPressed(KeyboardKey::KEY_SPACE as i32) {
                if color == Color::WHITE {
                    color = Color::PINK;
                } else if color == Color::PINK {
                    color = Color::WHITE;
                }
            }
        }

        let twid = rl.measure_text(thms.as_str(), (font_size as f64 * scale) as i32);
        let thi = (font_size as f64 * scale) as i32;

        cwid = rl.get_screen_width();
        chit = rl.get_screen_height();

        let mut pos = ffi::Vector2 {
            x: ((cwid - twid) / 2) as f32,
            y: ((chit - thi) / 2) as f32,
        };

        timemvmnt(vib_counter, &mut pos);

        camera.zoom = scale as f32;
        camera.offset = pos;
        camera.target = pos;

        vib_counter += 1;

        unsafe {
            ffi::BeginMode2D(camera);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text_ex(
            &font,
            thms.as_str(),
            pos,
            font_size as f32 * scale as f32,
            10.0,
            color,
        );
        unsafe {
            ffi::EndMode2D();
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mode = handle_args(&args);

    let font_path = "/home/m1nus/.fonts/Monaco.ttf";

    unsafe {
        ffi::SetConfigFlags(ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32);
    }

    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("tima").build();

    let font = rl.load_font(&thread, font_path).unwrap();
    let scale = 400.0;
    let font_size = 1;

    let cwid = rl.get_screen_width();
    let chit = rl.get_screen_height();

    let camera = ffi::Camera2D {
        offset: ffi::Vector2 {
            x: (cwid / 2) as f32,
            y: (chit / 2) as f32,
        },
        target: ffi::Vector2 {
            x: (cwid / 2) as f32,
            y: (chit / 2) as f32,
        },
        rotation: 0.0,
        zoom: 1.0,
    };

    rl.set_target_fps(FPS);

    match mode {
        Some(x) => match x.as_str() {
            "clock" => clock(rl, thread, camera, scale, font, font_size, cwid, chit),
            "timer" => timer(rl, thread, camera, scale, font, font_size, cwid, chit),
            _ => stopwatch(rl, thread, camera, scale, font, font_size, cwid, chit),
        },
        None => (),
    }
}
