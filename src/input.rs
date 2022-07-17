//! take sdl2 events and update a struct of bools for required controls

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

/// Holds mouse input info
#[derive(Copy, Clone)]
pub struct Mouse {
    pub x : i32,
    pub y : i32,
    pub left_click : bool,
    pub right_click : bool,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            x: 0,
            y: 0,
            left_click : false,
            right_click : false,
        }
    }
}


/// Holds character typed that frame, and the state of some useful buttons for typing
#[derive(Copy, Clone)]
pub struct Typing {
    pub character : Option<char>,
    pub backspace : bool,
    pub ctrl      : bool,
    pub shift     : bool,
    pub enter     : bool,
    pub tab       : bool,
    pub c         : bool,
    pub v         : bool,
    pub z         : bool,
    pub y         : bool,
    pub s         : bool,
    pub l         : bool,
    pub n         : bool,
    pub p         : bool,
    pub up        : bool,
    pub down      : bool,
    pub left      : bool,
    pub right     : bool,
    pub mouse     : Mouse,
}

impl Typing {

    pub fn new() -> Self {
        Typing {
            character: None,
            backspace: false,
            ctrl     : false,
            shift    : false,
            enter    : false,
            tab      : false,
            c        : false,
            v        : false,
            z        : false,
            y        : false,
            s        : false,
            l        : false,
            n        : false,
            p        : false,
            up       : false,
            down     : false,
            left     : false,
            right    : false,
            mouse    : Mouse::new(),
        }
    }

    pub fn used_character(&mut self) {
        self.character = None;
    }

    pub fn handle_event(&mut self, event: &Event) {
        if event.is_keyboard() {
            self.handle_keyboard(event);
        } else if event.is_text() {
            self.handle_text(event);
        } else if event.is_mouse() {
            self.handle_mouse(event);
        }
    }

    fn handle_keyboard(&mut self, event : &Event) {
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
                    Scancode::Tab => self.tab = key_down,
                    Scancode::C => self.c = key_down,
                    Scancode::V => self.v = key_down,
                    Scancode::Z => self.z = key_down,
                    Scancode::Y => self.y = key_down,
                    Scancode::S => self.s = key_down,
                    Scancode::L => self.l = key_down,
                    Scancode::N => self.n = key_down,
                    Scancode::P => self.p = key_down,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn handle_text(&mut self, event : &Event) {
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

    fn handle_mouse(&mut self, event : &Event) {
        let mut btn_down = false;
        let btn = match event {
            Event::MouseMotion { x, y, .. } => {
                self.mouse.x = *x;
                self.mouse.y = *y;
                None
            },
            Event::MouseButtonDown { mouse_btn, ..} => {
                btn_down = true;
                Some(mouse_btn)
            },
            Event::MouseButtonUp { mouse_btn, .. } => {
                btn_down = false;
                Some(mouse_btn)
            }
            _ => None,
        };
        match btn {
            Some(btn) => match btn {
                MouseButton::Left => self.mouse.left_click = btn_down,
                MouseButton::Right => self.mouse.right_click = btn_down,
                _ => (),
            }
            None => (),
        }
    }
}
