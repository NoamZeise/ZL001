//! Holds a list of microcontrollers and update/draws the currently active one

use crate::microcontroller::Microcontroller;
use crate::resource::Font;
use crate::geometry::Rect;
use crate::input::Typing;
use crate::{GameObject, FontManager, TextureManager};

use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Circuit<'a> {
    mc_game_obj : GameObject,
    active_mc : usize,
    mcs : Vec<Microcontroller<'a>>,
    mono_font : Font,
    prev_typing : Typing,
}

impl<'a> Circuit<'a> {
    pub fn new(mono_font : Font, mc_game_obj : GameObject) -> Self {
        Circuit {
            mc_game_obj,
            active_mc : 0,
            mcs: Vec::new(),
            mono_font,
            prev_typing : Typing::new(),
        }
    }
    
    pub fn add_circuit(&mut self, rect : Rect) {
        let mut game_obj = self.mc_game_obj.clone();
        game_obj.draw_rect = rect;
        self.mcs.push(Microcontroller::new(game_obj, self.mono_font.clone()));
        self.active_mc = self.mcs.len();
    }

    pub fn draw<TTex, TFont>(&mut self, canvas : &mut Canvas<Window>,  texture_manager : &'a TextureManager<TTex>, font_manager : &'a FontManager<TFont>) -> Result<(), String> {
        if self.active_mc < self.mcs.len() {
            self.mcs[self.active_mc].draw(canvas, font_manager)?;
        } else {
            for mc in self.mcs.as_slice() {
                texture_manager.draw(canvas, mc.get_game_object())?;
            }
        }
        Ok(())
    }

    /// update circuit or active `CodeWindow`
    pub fn update(&mut self, frame_elapsed : f64, typing : &mut Typing) {

        if typing.ctrl && typing.n && ! self.prev_typing.n {
            self.active_mc += 1;
        }

        if typing.ctrl && typing.p && ! self.prev_typing.p {
            if self.active_mc > 0 {
                self.active_mc -= 1;
            }
        }
        
        if self.active_mc < self.mcs.len() {
            self.mcs[self.active_mc].update(frame_elapsed, typing);
        }

        if typing.ctrl && typing.l && !self.prev_typing.l {
            for mc in self.mcs.as_mut_slice() {
                match mc.compile() {
                    Ok(_) => println!("Code OK"),
                    Err(_) => println!("Code Err"),
                }
            }
        }

        if typing.ctrl && typing.s && !self.prev_typing.s {
            for mc in self.mcs.as_mut_slice() {
                mc.step();
                mc.debug_print_registers();
            }
        }

        if typing.ctrl && typing.up && !self.prev_typing.up {
            for mc in self.mcs.as_mut_slice() {
                match mc.io_read_in(10, 2) {
                    Ok(_) => println!("read in value"),
                    Err(_) => println!("failed to read in value"),
                }
            }
        }

        if typing.ctrl && typing.down && !self.prev_typing.down {
            for mc in self.mcs.as_mut_slice() {
                match mc.io_read_out(3) {
                    Some(v) => println!("read {} from io 3", v),
                    None => println!("nothing to read from io 3"),
                }
            }
        }

        self.prev_typing = *typing;
    }
}
