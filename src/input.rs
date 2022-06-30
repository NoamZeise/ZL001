use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
