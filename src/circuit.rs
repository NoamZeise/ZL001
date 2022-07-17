//! Holds a list of microcontrollers and update/draws the currently active one

use crate::resource::Font;
use crate::geometry::Rect;
use crate::input::Typing;
use crate::{GameObject, FontManager, TextureManager,  microcontroller::Microcontroller, gui::Gui};

use sdl2::render::Canvas;
use sdl2::video::Window;

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub struct Circuit<'a> {
    mc_game_obj : GameObject,
    active_mc : usize,
    mcs : Vec<Microcontroller<'a>>,
    connections : HashMap<McConnection, McConnection>,
    mono_font : Font,
    prev_typing : Typing,
    gui : Gui,
}

impl<'a> Circuit<'a> {
    pub fn new(mono_font : Font, mc_game_obj : GameObject, btn_game_obj : GameObject) -> Self {
        Circuit {
            mc_game_obj,
            active_mc : 0,
            mcs: Vec::new(),
            connections : HashMap::new(),
            mono_font : mono_font.clone(),
            prev_typing : Typing::new(),
            gui : Gui::new(btn_game_obj, mono_font)
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
    pub fn add_connection(&mut self, mc_i1 : usize, io_i1 : usize, mc_i2 : usize, io_i2 : usize) -> Result<(), String> {
        if mc_i1 >= self.mcs.len() || mc_i2 >= self.mcs.len() {
            return Err(String::from("connection: mc index out of range"));
        }
        if io_i1 >= self.mcs[mc_i1].io_count() || io_i2 >= self.mcs[mc_i2].io_count() {
            return Err(String::from("connection: io index out of range"));
        }
        self.connections.insert(McConnection::new(mc_i1, io_i1), McConnection::new(mc_i2, io_i2));
        Ok(())
    }

    pub fn draw<TTex, TFont>(&mut self, canvas : &mut Canvas<Window>,  texture_manager : &'a TextureManager<TTex>, font_manager : &'a FontManager<TFont>) -> Result<(), String> {
        if self.active_mc < self.mcs.len() {
            self.mcs[self.active_mc].draw(canvas, font_manager)?;
        } else {
            for mc in self.mcs.as_slice() {
                texture_manager.draw(canvas, mc.get_game_object())?;
            }
            self.gui.draw(canvas, texture_manager, font_manager)?;
        }
        Ok(())
    }

    /// update circuit or active `CodeWindow`
    pub fn update(&mut self, frame_elapsed : f64, typing : &mut Typing) {

        if self.active_mc < self.mcs.len() {
            self.mcs[self.active_mc].update(frame_elapsed, typing);
        } else {
            self.circuit_controls(typing);
        }

        self.debug_controls(typing);
       
        self.prev_typing = *typing;
    }

    fn circuit_controls(&mut self, typing : &Typing) {
        self.gui.update(&typing.mouse);
        if let Some(rect) = self.gui.add_circ_request() {
            self.add_circuit(rect);
        }
    }
    
    fn io_in_ready(&self, connection : &McConnection) -> bool {
        self.mcs[connection.get_mc_i()].io_read_in_ready(connection.get_io_i())
    }

    fn io_out_ready(&self, connection : &McConnection) -> bool {
        self.mcs[connection.get_mc_i()].io_read_out_ready(connection.get_io_i())
    }

    fn step_circuit(&mut self) {
        let mut read_out_ports : Vec<McConnection> = Vec::new();
        for (mc_i, mc) in self.mcs.as_mut_slice().into_iter().enumerate() {
            mc.step();
            mc.debug_print_registers();
            for port_i in 0..mc.io_count() {
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
                        if self.io_in_ready(io_in) && self.io_out_ready(io_out) {
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

    /// save the circuit to given file path
    pub fn save_to_file(&self, path : &Path) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        //save mcs
        for mc in self.mcs.as_slice() {
            let mc_game_obj = mc.get_game_object();
            file.write(
                format!(
                    "<mc>\n{} {} {} {}\n",
                    mc_game_obj.draw_rect.x,
                    mc_game_obj.draw_rect.y,
                    mc_game_obj.draw_rect.w,
                    mc_game_obj.draw_rect.h,
                ).as_bytes()
            ).map_err(|e| e.to_string())?;
            file.write(mc.get_code().as_bytes()).map_err(|e| e.to_string())?;
            file.write("\n".as_bytes()).map_err(|e| e.to_string())?;
        }
        //save mc connections
        file.write("<connections>\n".as_bytes()).map_err(|e| e.to_string())?;
        for (out_io, in_io) in self.connections.iter() {
             file.write(
                format!(
                    "{} {} {} {}\n",
                    out_io.get_mc_i(),
                    out_io.get_io_i(),
                    in_io.get_mc_i(),
                    in_io.get_io_i(),
                ).as_bytes()
            ).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// clear current circuit and load previously saved circuit
    pub fn load_from_file(&mut self, path : &Path) -> Result<(), String> {
        //clear circuit
        self.mcs.clear();
        self.connections.clear();
        self.active_mc = 0;

        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut text = String::new();
        file.read_to_string(&mut text).map_err(|e| e.to_string())?;
        let (mc_text, connection_text) = match text.split_once("<connections>") {
            Some(v) => v,
            None => { return Err(String::from("error parsing <connections>")); },
        };
        //load mcs
        for mc in mc_text.split("<mc>").skip(1) {
            //get rect
            let (rect, code) = match mc.trim_start().split_once("\n") {
                Some(v) => v,
                None => { return Err(String::from("error parsing rect/code split")); },
            };
            let rect = parse_4_vals(rect.trim())?;
            let rect = Rect::new(rect[0], rect[1], rect[2], rect[3]);
            self.add_circuit(rect);
            self.mcs.last_mut().unwrap().set_code(code.to_string());
        }

        //load mc connections
        for con in connection_text.trim().split("\n") {
            let con = parse_4_vals(con)?;
            self.add_connection(con[0], con[1], con[2], con[3])?;
        }
        
        Ok(())
    }

    fn debug_controls(&mut self, typing : &mut Typing) {
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
            self.save_to_file(Path::new("saves/test.circ")).unwrap();
        }

        if typing.ctrl && typing.down && !self.prev_typing.down {
            self.load_from_file(Path::new("saves/test.circ")).unwrap();
        }

        if typing.ctrl && typing.c && !self.prev_typing.c {
            self.connections.clear();
            self.mcs.clear();
        }
    }
}

fn parse_4_vals<T : FromStr>(text : &str) -> Result<Vec::<T>, String> 
where <T as FromStr>::Err : std::fmt::Debug {
    let mut vals : Vec<T> = Vec::new();
    let vals_result : Vec<Result<T, String>> =
        text
        .split(" ")
        .map(
            |v|
            v.parse::<T>().map_err(|_| String::from("error parsing str into number [circuit::parse_4_vals()]"))
        )
        .collect();
    for v in vals_result {
        vals.push(v?)
    }
    if vals.len() != 4 {
        return Err(String::from("parse wasn't 4"));
    }
    Ok(vals)
}

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
