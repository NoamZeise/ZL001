use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image;

use zl001::{TextureManager, FontManager, GameObject};
use zl001::input::{Input, Typing};
use zl001::circuit::Circuit;
use zl001::geometry::Rect;

use std::time::Instant;
use std::path::Path;

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
    
    let mut circuit = Circuit::new(
        mono_font,
        GameObject::new(
            texture_manager.load(
                Path::new("textures/microcontroller.png")
            )?
        )
    );
    circuit.add_circuit(Rect::new(20.0, 50.0, 100.0, 100.0));
    circuit.add_circuit(Rect::new(300.0, 50.0, 100.0, 100.0));
    circuit.add_circuit(Rect::new(20.0, 400.0, 100.0, 100.0));
    circuit.add_connection(0, 0, 1, 0);
    circuit.add_connection(1, 0, 0, 0);
    circuit.add_connection(0, 1, 2, 0);
    circuit.add_connection(2, 0, 0, 1);

    canvas.set_draw_color(Color::RGB(45, 59, 55));

    video_subsystem.text_input().start();

    let mut event_pump = sdl_context.event_pump()?;
    let mut input = Input::new();
    let mut typing = Typing::new();
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

        //draw
        circuit.draw(&mut canvas, &texture_manager, &font_manager)?;
        
        canvas.present();

        //update
        circuit.update(prev_frame, &mut typing);

        prev_frame = start_time.elapsed().as_secs_f64();
    }

    Ok(())
}
