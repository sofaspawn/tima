use ffi::IsKeyDown;
use rand::Rng;
use std::{env, error::Error};
use raylib::prelude::*;
use chrono::prelude::*;

const WIDTH:i32 = 2000;
const HEIGHT:i32 = 1000;

fn elapsedtime(anchor: DateTime<Local>)->String{
    let elapsed = Local::now() - anchor;
    let ehours = elapsed.num_hours();
    let eminutes = elapsed.num_minutes();
    let eseconds = elapsed.num_seconds();

    let hdiv = (ehours/60) as i64;
    let mdiv = (eminutes/60) as i64;
    let sdiv = (eseconds/60) as i64;

    let ohours = if (ehours - hdiv*60)<10{format!("0{}", ehours - hdiv*60)} else if hdiv>0{(ehours - hdiv*60).to_string()} else {ehours.to_string()};
    let ominutes = if (eminutes - mdiv*60)<10{format!("0{}", eminutes - mdiv*60)} else if mdiv>0{(eminutes - mdiv*60).to_string()} else {eminutes.to_string()};
    let oseconds = if (eseconds - sdiv*60)<10{format!("0{}", eseconds - sdiv*60)} else if sdiv>0{(eseconds - sdiv*60).to_string()} else {eseconds.to_string()};

    format!("{}:{}:{}", ohours, ominutes, oseconds)
}

fn timemvmnt(counter:i32, pos: &mut ffi::Vector2){
    //agressive shaking
    if counter%1==0{ // counter for customization
        pos.x += rand::thread_rng().gen_range(-20..21) as f32;
        pos.y += rand::thread_rng().gen_range(-20..21) as f32;
    }
}

fn handle_args(args: Vec<String>)->Option<String>{
    if args.len()>2{
        return None;
    } else if args.len()==2 {
        return Some(args[1].clone());
    } else {
        return Some(args[0].clone());
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mode = handle_args(args);
    println!("{}",mode.unwrap_or("Crashed".to_string()));
    let font_path = "/home/m1nus/.fonts/Monaco.ttf";

    unsafe{
        ffi::SetConfigFlags(ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32);
    }

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("tima")
        .build();

    let font = rl.load_font(&thread, font_path).unwrap();

    let mut scale = 400.0;
    let font_size = 1;
    let init = Local::now();

    let mut cwid = rl.get_screen_width();
    let mut chit = rl.get_screen_height();

    let mut camera = ffi::Camera2D{
        offset: ffi::Vector2{x:(cwid/2) as f32, y:(chit/2) as f32}, 
        target: ffi::Vector2{x:(cwid/2) as f32, y:(chit/2) as f32},
        rotation: 0.0,
        zoom: 1.0
    };

    rl.set_target_fps(60);

    let mut vib_counter = 0;

    while !rl.window_should_close() {
        let thms = elapsedtime(init);

        unsafe{
            if IsKeyDown(KeyboardKey::KEY_Q as i32){
                break;
            }
            if IsKeyDown(KeyboardKey::KEY_EQUAL as i32){
                scale+=2.0;
                //println!("{scale}");
            }
            if IsKeyDown(KeyboardKey::KEY_MINUS as i32){
                if scale>5.0{
                    scale-=2.0;
                }
                //println!("{scale}");
            }
            if IsKeyDown(KeyboardKey::KEY_ZERO as i32){
                scale = 400.0;
            }
        }

        let twid = rl.measure_text(thms.as_str(), (font_size as f32 * scale) as i32);
        let thi = (font_size as f32 * scale) as i32;

        cwid = rl.get_screen_width();
        chit = rl.get_screen_height();

        let mut pos = ffi::Vector2{
            x:((cwid - twid)/2) as f32,
            y:((chit-thi)/2) as f32,
        };

        timemvmnt(vib_counter, &mut pos);

        camera.zoom = scale as f32;
        //camera.target = ffi::Vector2{x:(cwid/2) as f32, y:(chit/2) as f32};
        //camera.offset = ffi::Vector2{x:(cwid/2) as f32, y:(chit/2) as f32};
        camera.offset = pos;
        camera.target = pos;

        vib_counter+=1;

        unsafe{
            ffi::BeginMode2D(camera);
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text_ex(&font, thms.as_str(), pos, font_size as f32 * scale, 10.0, Color::WHITE);
        unsafe{
            ffi::EndMode2D();
        }
    }
}
