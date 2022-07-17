//! Holds a `Program` and `CodeWindow` for inputting, drawing and executing user code

mod assembler;
mod code_window;
mod program;

use self::assembler::CodeError;
use self::code_window::CodeWindow;
use self::program::Program;

use crate::geometry::*;
use crate::resource::Font;
use crate::input::Typing;
use crate::FontManager;
use crate::GameObject;

use sdl2::video::Window;
use sdl2::render::Canvas;

/// has an interface for updating an drawing a `CodeWindow` and executing a `Program`
pub struct Microcontroller<'a> {
    game_obj : GameObject,
    code_window : CodeWindow<'a>,
    program : Program,
}

impl<'a> Microcontroller<'a> {

    /// make a new mc at a location with a font for rendering the `CodeWindow`
    pub fn new(game_obj : GameObject, font : Font) -> Self {
        Microcontroller {
            game_obj,
            code_window : CodeWindow::new(font, Vec2::new(20.0, 10.0)),
            program : Program::blank(),
        }
    }

    /// get program io count
    pub fn io_count(&self) -> usize {
        self.program.io_reg_count()
    }
    
/// draw the `CodeWindow` to the sdl2 `Canvas`
    pub fn draw<T>(&mut self, canvas: &mut Canvas<Window>, font_manager : &'a FontManager<T>) -> Result<(), String> {
        self.code_window.set_draw_lines(font_manager)?;
        for l in self.code_window.get_draw_code() {
            canvas.copy(&l.tex, None, l.rect)?;
        }
        Ok(())
    }

    pub fn get_game_object(&self) -> &GameObject {
        &self.game_obj
    }
/// Update the `CodeWindow` with user input
    pub fn update(&mut self, frame_elapsed : f64, typing : &mut Typing) {
        self.code_window.update(frame_elapsed, typing)
    }
/// Run the `assembler` on the code inputted to the `CodeWindow` and store as a `Program`    
    pub fn compile(&mut self) -> Result<(), CodeError> {
        Ok(self.program = Program::new(self.code_window.get_code())?)
    }

    /// get current code stored in `CodeWindow`
    pub fn get_code(&self) -> &str {
        self.code_window.get_code()
    }

    /// set current code stored in [`CodeWindow`]
    pub fn set_code(&mut self, code : String) {
        self.code_window.set_code(code);
    }

    /// excute the next instruction in the `Program`
    pub fn step(&mut self) {
        self.program.step();
    }

    /// read value to io register at index
    pub fn io_read_in(&mut self, value : i16, index : usize) -> Result<(), ()>  {
        self.program.read_in(value, index)
    }

    pub fn io_read_in_ready(&self, index : usize) -> bool {
        self.program.read_in_ready(index)
    }

    /// read value from io register at index
    pub fn io_read_out(&mut self, index : usize) -> Option<i16> {
        self.program.read_out(index)
    }

    pub fn io_read_out_ready(&self, index : usize) -> bool {
        self.program.read_out_ready(index)
    }

    /// debug function to show value in registers
    #[cfg(debug_assertions)]
    pub fn debug_print_registers(&self) {

        if self.program.halted() {
            println!("Program Halted\n");
        } else {
            println!("\nPC: {}", self.program.get_register_value(assembler::Register::PC).unwrap());
            println!("R1: {}", self.program.get_register_value(assembler::Register::R1).unwrap());
            println!("R2: {}", self.program.get_register_value(assembler::Register::R2).unwrap());
            println!("RT: {}", self.program.get_register_value(assembler::Register::RT).unwrap());
            for io_reg in 0..assembler::IO_REGISTER_COUNT {
                if self.program.read_in_ready(io_reg) {
                    println!("accepting RI[{}]", io_reg);
                }
                if self.program.read_out_ready(io_reg) {
                    println!("waiting   RO[{}]", io_reg, );
                }
            }
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn debug_print_registers(&self) {
    }
    
}
