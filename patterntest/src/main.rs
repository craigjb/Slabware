use std::time::Instant;
use {
    sdl2, sdl2::event::Event, sdl2::keyboard::Keycode, sdl2::pixels::PixelFormatEnum,
    sdl2::render::WindowCanvas, sdl2::video::DisplayMode,
};

const WIDTH: usize = 1152;
const HEIGHT: usize = 512;

fn main() {
    let sdl_context = sdl2::init().expect("Failed to init SDL");
    let sdl_video = sdl_context.video().expect("Failed to init SDL video");
    let mut window = sdl_video
        .window("cgmath demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .expect("Failed to create window");
    window
        .set_display_mode(DisplayMode::new(PixelFormatEnum::RGBA8888, 1024, 768, 60))
        .expect("Failed to set display mode");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");

    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");
    let start = Instant::now();
    loop {
        // clear to all black
        canvas.set_draw_color((0, 0, 0, 0));
        canvas.clear();

        render(&mut canvas, start.elapsed().as_nanos() as f32 / 1.0e9);

        // show changes on the screen
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return,
                _ => {}
            }
        }
    }
}

fn render(canvas: &mut WindowCanvas, time: f32) {
    const SCALE: f64 = 2.0 * std::f64::consts::PI / 128.0;
    let lut: Vec<u8> = (0..256)
        .map(|i| ((i as f64 * SCALE).sin() * 255.0) as u8)
        .collect();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let utime = (time * 100.0) as usize;
            let xmod = (x + utime) % 64;
            let r = if xmod >= 32 { 63 - xmod } else { xmod };

            let ymod = ((y >> 1) + utime) % 64;
            let b = if ymod >= 32 { 63 - ymod } else { ymod };

            let xymod = (x - y + utime) % 128;
            let g = if xymod >= 64 { 127 - xymod } else { xymod };

            canvas.set_draw_color(((r << 3) as u8, (g << 2) as u8, (b << 3) as u8));
            canvas.draw_point((x as i32, y as i32)).unwrap();
        }
    }
}
