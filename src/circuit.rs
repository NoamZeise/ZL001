//! Holds a list of microcontrollers and update/draws the currently active one

use crate::microcontroller::Microcontroller;
use crate::resource::Font;
use crate::geometry::Rect;
use crate::input::Typing;
use crate::{GameObject, FontManager, TextureManager};

use sdl2::render::Canvas;
use sdl2::video::Window;

use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
struct McConnection {
    mc_i : usize,
    io_i : usize,
}


impl McConnection {
    fn new(mc_i : usize, io_i : usize) -> Self {
        McConnection { mc_i, io_i }
    }

    fn get_mc_i(&self) -> usize {
        self.mc_i
    }
    
    fn get_io_i(&self) -> usize {
        self.io_i
    }
}

pub struct Circuit<'a> {
    mc_game_obj : GameObject,
    active_mc : usize,
    mcs : Vec<Microcontroller<'a>>,
    connections : HashMap<McConnection, McConnection>,
    mono_font : Font,
    prev_typing : Typing,
}

impl<'a> Circuit<'a> {
    pub fn new(mono_font : Font, mc_game_obj : GameObject) -> Self {
        Circuit {
            mc_game_obj,
            active_mc : 0,
            mcs: Vec::new(),
            connections : HashMap::new(),
            mono_font,
            prev_typing : Typing::new(),
        }
    }

    /// temp function until UI working -> add mc to circuit at rect location
    pub fn add_circuit(&mut self, rect : Rect) {
        let mut game_obj = self.mc_game_obj.clone();
        game_obj.draw_rect = rect;
        self.mcs.push(Microcontroller::new(game_obj, self.mono_font.clone()));
        self.active_mc = self.mcs.len();
    }

    /// temp function until UI working -> add connection between two mcs to circuit
    pub fn add_connection(&mut self, mc_i1 : usize, io_i1 : usize, mc_i2 : usize, io_i2 : usize) {
        self.connections.insert(McConnection::new(mc_i1, io_i1), McConnection::new(mc_i2, io_i2));
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

    fn step_circuit(&mut self) {
        let mut read_out_ports : Vec<McConnection> = Vec::new();
        for (mc_i, mc) in self.mcs.as_mut_slice().into_iter().enumerate() {
            mc.step();
            mc.debug_print_registers();
            for port_i in 0..crate::assembler::IO_REGISTER_COUNT {
                if mc.io_read_out_ready(port_i) {
                    read_out_ports.push(McConnection::new(mc_i, port_i));
                }
            }
        }
        //read out until no more read_outs left
        loop {
            let mut read_out_val = false;
            for io_out in read_out_ports.as_slice().into_iter() {
                match self.connections.get(&io_out) {
                    Some(io_in) => {
                        if self.mcs[io_in.get_mc_i()].io_read_in_ready(io_in.get_io_i()) &&
                           self.mcs[io_out.get_mc_i()].io_read_out_ready(io_out.get_io_i()){
                            let value = self.mcs[io_out.get_mc_i()].io_read_out(io_out.get_io_i()).unwrap();
                            self.mcs[io_in.get_mc_i()].io_read_in(value, io_in.get_io_i()).unwrap();
                            //step for read in mc to complete instruction
                            self.mcs[io_in.get_mc_i()].step();
                            read_out_val = true;
                        }
                    },
                    None => (),
                }
            }
            if !read_out_val {
                break;
            }
        }
    }

    /// update circuit or active `CodeWindow`
    pub fn update(&mut self, frame_elapsed : f64, typing : &mut Typing) {

        if typing.ctrl && typing.n && ! self.prev_typing.n {
            if self.active_mc < self.mcs.len() {
                self.active_mc += 1;
            }
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
            self.step_circuit();
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
