use ffi::IsKeyDown;
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

/*
fn timemvmnt(pos: ffi::Vector2)->ffi::Vector2{
    let mut new_pos = pos.clone();
    new_pos.x += 1.0;
    new_pos.y += 1.0;
    new_pos
}
*/

fn main() {
    let font_path = "/home/m1nus/.fonts/Monaco.ttf";

    unsafe{
        ffi::SetConfigFlags(ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32);
    }

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("tima")
        .build();

    let font = rl.load_font(&thread, font_path).unwrap();

    let mut scale = 1.0;
    let font_size = 40;
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

    while !rl.window_should_close() {
        let thms = elapsedtime(init);

        unsafe{
            if IsKeyDown(KeyboardKey::KEY_Q as i32){
                break;
            }
            if IsKeyDown(KeyboardKey::KEY_EQUAL as i32){
                scale+=0.2;
                //println!("{scale}");
            }
            if IsKeyDown(KeyboardKey::KEY_MINUS as i32){
                if scale>0.2{
                    scale-=0.2;
                }
                //println!("{scale}");
            }
        }

        let twid = rl.measure_text(thms.as_str(), font_size);
        let thi = font_size;

        cwid = rl.get_screen_width();
        chit = rl.get_screen_height();

        let pos = ffi::Vector2{
            x:((cwid - twid)/2) as f32,
            y:((chit-thi)/2) as f32,
        };

        println!("{scale}");
        camera.zoom = scale as f32;
        //camera.target = ffi::Vector2{x:(cwid/2) as f32, y:(chit/2) as f32};
        //camera.offset = ffi::Vector2{x:(cwid/2) as f32, y:(chit/2) as f32};
        camera.offset = pos;
        camera.target = pos;

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
