use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use std::path::Path;

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

    canvas.set_draw_color(Color::RGB(100, 100, 100));

    let mut event_pump = sdl_context.event_pump()?;

    let gaia_tex = texture_creator.load_texture(Path::new("textures/gaia.png"))?;


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();

        canvas.copy(
            &gaia_tex,
            None,
            Rect::new
                (
                    100,
                    100,
                    gaia_tex.query().width  * 4,
                    gaia_tex.query().height * 4
                )
        )?;

        canvas.present();

        // The rest of the game loop goes here...
    }

    Ok(())
}
