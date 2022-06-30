use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image;

use zl001::TextureManager;
pub mod player;
use player::Player;
pub mod input;
use input::Input;

use std::path::Path;
use std::time::Instant;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(image::InitFlag::PNG);
    let window = video_subsystem
        .window("ZL001", 1600, 900)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);

    canvas.set_draw_color(Color::RGB(100, 100, 100));

    let mut event_pump = sdl_context.event_pump()?;
    let mut input = Input::new();
    let mut player = Player::new(texture_manager.load(Path::new("textures/gaia.png"))?);
    let mut prev_frame : f64 = 0.0;
    'running: loop {
        let start_time = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
            input.handle_event(&event);
        }

        canvas.clear();

        texture_manager.draw(&mut canvas, &player.game_obj())?;

        canvas.present();

        // The rest of the game loop goes here...
        player.update(prev_frame, &input);

        prev_frame = start_time.elapsed().as_secs_f64();
    }

    Ok(())
}
