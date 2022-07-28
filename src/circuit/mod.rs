//! Holds a list of microcontrollers and update/draws the currently active one

mod circuit_helper;
mod circuit_gui;
mod code_gui;
mod button;

use crate::resource::Font;
use crate::geometry::Rect;
use crate::input::Typing;
use crate::{GameObject, FontManager, TextureManager,  microcontroller::Microcontroller};

use circuit_helper::McConnection;
use circuit_gui::Gui;
use code_gui::CodeGui;

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
    code_gui : CodeGui,
    modified : bool,
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
            gui : Gui::new(btn_game_obj.clone(), mono_font.clone()),
            code_gui : CodeGui::new(btn_game_obj, mono_font),
            modified : true,
        }
    }

    fn add_circuit(&mut self, rect : Rect) {
        let mut game_obj = self.mc_game_obj.clone();
        game_obj.draw_rect = rect;
        self.mcs.push(Microcontroller::new(game_obj, self.mono_font.clone()));
        self.active_mc = self.mcs.len();
        self.modified = true;
    }

    fn add_connection(&mut self, con1 : McConnection, con2 : McConnection) -> Result<(), String> {
        if con1.get_mc_i() >= self.mcs.len() || con2.get_mc_i() >= self.mcs.len() {
            return Err(String::from("connection: mc index out of range"));
        }
        if con1.get_io_i() >= self.mcs[con1.get_mc_i()].io_count() || con2.get_io_i() >= self.mcs[con2.get_mc_i()].io_count() {
            return Err(String::from("connection: io index out of range"));
        }
        self.connections.insert(con1, con2);
        self.modified = true;
        Ok(())
    }

    pub fn draw<TTex, TFont>(&mut self, canvas : &mut Canvas<Window>,  texture_manager : &'a TextureManager<TTex>, font_manager : &'a FontManager<TFont>) -> Result<(), String> {
        if self.active_mc < self.mcs.len() {
            self.mcs[self.active_mc].draw(canvas, font_manager)?;
            self.code_gui.draw(canvas, texture_manager, font_manager)?;
        } else {
            self.gui.draw(canvas, texture_manager, font_manager)?;
        }
        Ok(())
    }

    /// update circuit or active `CodeWindow`
    pub fn update(&mut self, frame_elapsed : f64, typing : &mut Typing) {
        if self.active_mc < self.mcs.len() {
            self.code_controls(frame_elapsed, typing);
        } else {
            self.circuit_controls(typing);
        }

        self.debug_controls(typing);
       
        self.prev_typing = *typing;
    }

    fn circuit_controls(&mut self, typing : &Typing) {
        self.gui.update(&typing.mouse, &self.mcs, &self.connections, self.modified);
        self.modified = false;
        if let Some(rect) = self.gui.add_circ_request() {
            self.add_circuit(rect);
        }
        //do both way connection
        if let Some((con1, con2)) = self.gui.add_con_request() {
            if !self.connections.contains_key(&con1) && !self.connections.contains_key(&con2) {
                self.add_connection(con1, con2).unwrap();
                self.add_connection(con2, con1).unwrap();
            }
        }
        if let Some(i) = self.gui.remove_mcs_index() {
            self.remove_connection(i);
            self.mcs.swap_remove(i);
            self.modified = true;
        }
        if let Some(i) = self.gui.code_mcs_index() {
            self.active_mc = i;
        }

        if self.gui.save_circuit() {
            self.save_to_file(Path::new("saves/test.circ")).unwrap();
        }

        if self.gui.load_circuit() {
            self.load_from_file(Path::new("saves/test.circ")).unwrap();
            self.modified = true;
        }

        if self.gui.clear_circuit() {
            self.connections.clear();
            self.mcs.clear();
            self.modified = true;
        }

        if self.gui.compile() {
            for mc in self.mcs.as_mut_slice() {
                match mc.compile() {
                    Ok(_) => println!("Code OK"),
                    Err(_) => println!("Code Err"),
                }
            }
        }

        if self.gui.step() {
            self.step_circuit();
        }
    }

    fn code_controls(&mut self, frame_elapsed : f64, typing : &mut Typing) {
        self.mcs[self.active_mc].update(frame_elapsed, typing);
        self.code_gui.update(&typing.mouse);

        if self.code_gui.circuit_btn() {
            self.active_mc = self.mcs.len();
        }
    }

    fn remove_connection(&mut self, i : usize) {
        if self.mcs.len() == 0 { return (); }
        let mut to_remove : Vec<McConnection> = Vec::new();
        let mut change_keys : Vec<McConnection> = Vec::new();
        let changed_index = self.mcs.len() - 1;
        
        for con in self.connections.iter() {
            if con.0.get_mc_i() == i || con.1.get_mc_i() == i {
                    to_remove.push(*con.0);
            }
            else if changed_index != i {
                if con.0.get_mc_i() == changed_index || con.1.get_mc_i() == changed_index{
                    change_keys.push(*con.0);
                }
            }
        }
        for r in to_remove {
            self.connections.remove(&r);
            }
        for k in change_keys {
            let mut v : McConnection = *self.connections.get(&k).unwrap();
            let mut new_k : McConnection = k;
            if new_k.get_mc_i() == changed_index {
                new_k = McConnection::new(i, new_k.get_io_i());
            }
            if v.get_mc_i() == changed_index {
                v = McConnection::new(i, v.get_io_i());
            }
            self.connections.remove(&k);
            self.connections.insert(new_k, v);
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
            if con != "" {
                let con = parse_4_vals(con)?;
                self.add_connection(McConnection::new(con[0], con[1]), McConnection::new(con[2], con[3]))?;
            }
        }
        self.modified = true;
        Ok(())
    }

    fn debug_controls(&mut self, typing : &mut Typing) {
        if typing.ctrl && typing.z && !self.prev_typing.z {
            println!("printing connections: ");
            for con in self.connections.iter() {
                println!("mc: {}, io: {} -> mc: {}, io: {}",
                         con.0.get_mc_i(), con.0.get_io_i(),
                         con.1.get_mc_i(), con.1.get_io_i()
                );
            }
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
            v.trim().parse::<T>().map_err(|_| String::from(format!("error parsing str into number [circuit::parse_4_vals()], text : {}", v.trim())))
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
