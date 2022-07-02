//! take sdl2 events and update a struct of bools for required controls

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

pub struct Input {
    pub up : bool,
    pub down : bool,
    pub left : bool,
    pub right : bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            up:    false,
            down:  false,
            left:  false,
            right: false,
        }
    }
    pub fn handle_event(&mut self, event: &Event) {
        if event.is_keyboard() {
           let mut key_down = false;
           let key = match event {
               Event::KeyDown {
                   keycode: k,
                   ..
               } => {
                   key_down = true;
                   k
               },
               Event::KeyUp {
                   keycode: k,
                   ..
               } => k,
                _ => &None
           };
           match key {
               Some(k) => {
                   match k {
                       Keycode::W => self.up    = key_down,
                       Keycode::A => self.left  = key_down,
                       Keycode::S => self.down  = key_down,
                       Keycode::D => self.right = key_down,
                       _ => {}
                   }
                }
               _ => {}
           }
        }
    } 
}

/// Holds character typed that frame, and the state of some useful buttons for typing
pub struct Typing {
    pub character : Option<char>,
    pub backspace : bool,
    pub ctrl      : bool,
    pub shift     : bool,
    pub enter     : bool,
    pub c         : bool,
    pub v         : bool,
    pub z         : bool,
    pub y         : bool,
    pub up        : bool,
    pub down      : bool,
    pub left      : bool,
    pub right     : bool,
}

impl Typing {

    pub fn new() -> Self {
        Typing {
            character: None,
            backspace: false,
            ctrl     : false,
            shift    : false,
            enter    : false,
            c        : false,
            v        : false,
            z        : false,
            y        : false,
            up       : false,
            down     : false,
            left     : false,
            right    : false,
        }
    }

    pub fn used_character(&mut self) {
        self.character = None;
    }

    pub fn handle_event(&mut self, event: &Event) {
     if event.is_keyboard() {
           let mut key_down = false;
           let key = match event {
               Event::KeyDown {
                   scancode: k,
                   ..
               } => {
                   key_down = true;
                   k
               },
               Event::KeyUp {
                   scancode: k,
                   ..
               } => k,
                _ => &None
           };
           match key {
               Some(k) => {
                   match k {
                       Scancode::Up => self.up    = key_down,
                       Scancode::Left => self.left  = key_down,
                       Scancode::Down => self.down  = key_down,
                       Scancode::Right => self.right = key_down,
                       Scancode::Backspace => self.backspace = key_down,
                       Scancode::Return => self.enter = key_down,
                       Scancode::LCtrl => self.ctrl = key_down,
                       Scancode::LShift => self.shift = key_down,
                       Scancode::C => self.c = key_down,
                       Scancode::V => self.v = key_down,
                       Scancode::Z => self.z = key_down,
                       Scancode::Y => self.y = key_down,
                       _ => {}
                   }
                }
               _ => {}
           }
        } else if event.is_text() {
         self.character = match event {
             Event::TextInput { text : t, ..} => {
                 if t.len() > 0 {
                     Some(t.chars().nth(0).unwrap())
                 } else {
                     None
                 }
             },
             _ => None,
         }
        }
    }
}
