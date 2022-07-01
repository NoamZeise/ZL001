use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image;

use zl001::{TextureManager, FontManager};
use zl001::player::Player;
use zl001::input::{Input, Typing};
use zl001::geometry::Vec2;

use std::path::Path;
use std::time::Instant;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(image::InitFlag::PNG);
    let window = video_subsystem
        .window("ZL001", 640, 480)
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
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font_manager = FontManager::new(&ttf_context, &texture_creator)?;

    let mono_font = font_manager.load_font(Path::new("textures/FiraCode-Light.ttf"))?;

    let mut message_texture = font_manager.get_draw(&mono_font, "Hello SDL2 Rust", 25)?;
    message_texture.rect.x = 10;
    message_texture.rect.y = 50;

    canvas.set_draw_color(Color::RGB(100, 100, 100));

    video_subsystem.text_input().start();

    let mut typed_text = String::new();

    let mut event_pump = sdl_context.event_pump()?;
    let mut input = Input::new();
    let mut typing = Typing::new();
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
            typing.handle_event(&event);
        }

        canvas.clear();

        texture_manager.draw(&mut canvas, &player.game_obj())?;

        canvas.copy(&message_texture.tex, None, message_texture.rect)?;

        font_manager.draw(&mut canvas, &mono_font, &prev_frame.to_string(), 100, Vec2::new(50.0, 200.0))?;

        match typing.character {
            Some(c) => {
                typed_text.push(c);
            },
            None => (),
        }
        typing.used_character();

        font_manager.draw(&mut canvas, &mono_font, &&typed_text, 55, Vec2::new(10.0, 300.0))?;

        canvas.present();

        // The rest of the game loop goes here...
        player.update(prev_frame, &input);

        prev_frame = start_time.elapsed().as_secs_f64();
    }

    Ok(())
}
