use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image;

use zl001::{TextureManager, FontManager};
use zl001::input::{Input, Typing};
use zl001::code_window::CodeWindow;
use zl001::program::Program;
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
    let _texture_manager = TextureManager::new(&texture_creator);
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font_manager = FontManager::new(&ttf_context, &texture_creator)?;

    let mono_font = font_manager.load_font(Path::new("textures/FiraCode-Light.ttf"))?;
    let mut code_window = CodeWindow::new(mono_font, Vec2::new(10.0, 5.0));
    let mut code = Program::blank();
    canvas.set_draw_color(Color::RGB(100, 100, 100));

    video_subsystem.text_input().start();

    let mut event_pump = sdl_context.event_pump()?;
    let mut input = Input::new();
    let mut typing = Typing::new();
    let mut prev_typing = Typing::new();
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

        code_window.set_draw_lines(&font_manager)?;
        for l in code_window.get_draw_code() {
            canvas.copy(&l.tex, None, l.rect)?;
        }

        canvas.present();

        code_window.update(prev_frame, &mut typing);

        if typing.ctrl && typing.l && !prev_typing.l {
            code = match Program::new(code_window.get_code()) {
                Ok(c) => {
                    println!("Code Ok");
                    c
                },
                Err(_) => {
                    println!("Code Err");
                    Program::blank()
                },
            };
        }

        if typing.ctrl && typing.s && ! prev_typing.s {
            code.step();
            if code.halted() {
                println!("Program Halted\n");
            } else {
                println!("PC: {}", code.get_register_value(zl001::assembler::Register::PC).unwrap());
                println!("R1: {}", code.get_register_value(zl001::assembler::Register::R1).unwrap());
                println!("R2: {}", code.get_register_value(zl001::assembler::Register::R2).unwrap());
                println!("RT: {}", code.get_register_value(zl001::assembler::Register::RT).unwrap());
                println!("accepting RI: {}", code.read_in_ready());
                println!("waiting   RO: {}\n", code.read_out_ready());
            }
        }

        if typing.ctrl && typing.up && ! prev_typing.up {
            match code.read_out() {
                Some(v) => println!("read out {}", v),
                None => println!("no value to read out"),
            }
        }

         if typing.ctrl && typing.down && ! prev_typing.down {
            match code.read_in(10) {
                Ok(_) => println!("accepted a value to in register"),
                Err(_) => println!("did not accept a value to in register"),
            }
        }
        
        
        prev_typing = typing;
        prev_frame = start_time.elapsed().as_secs_f64();
    }

    Ok(())
}
