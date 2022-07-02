use crate::TextDraw;
use crate::input::Typing;
use crate::FontManager;
use crate::resource::Font;
use crate::geometry::Vec2;

use std::iter::Iterator;

const FONT_MONO_WIDTH : f64 = 12.5;
const TEXT_HEIGHT : u32 = 35;
const BACKSPACE_DELAY : f64 = 0.6;
const BACKSPACE_REPEAT_SPEED : f64 = 0.05;

pub struct CodeWindow<'a> {
    code : String,
    code_lines : Vec<String>,
    line_draws : Vec<TextDraw<'a>>,
    code_changed : bool,
    since_backspace : f64,
    backspace_pressed : bool,
    enter_pressed : bool,
    mono_font : Font,
}


impl<'a> CodeWindow<'a> {
    pub fn new(mono_font : Font) -> Self {
        CodeWindow {
            code: String::new(),
            code_lines: Vec::new(),
            line_draws: Vec::new(),
            since_backspace : BACKSPACE_DELAY,
            backspace_pressed : false,
            enter_pressed : false,
            code_changed : false,
            mono_font,
        }
    }

    pub fn update(&mut self, frame_elapsed: f64, typing: &mut Typing) {
        self.code_changed = false;
        self.since_backspace += frame_elapsed;
        if !typing.ctrl  && !typing.backspace && !typing.enter {
            self.since_backspace = BACKSPACE_DELAY;
            self.backspace_pressed = false;
            self.enter_pressed = false;
            match typing.character {
                Some(c) => {
                    self.code_changed = true;
                    self.code.push(c)
                },
                None => (),
            }
            typing.used_character();
        } else {
                   if typing.c {
                //copy
            } else if typing.v {
                //paste
            } else if typing.z {
                //undo
            } else if typing.y {
                //redo
            }
            if typing.enter && !self.enter_pressed {
                self.enter_pressed = true;
                self.code_changed = true;
                self.code.push('\n');
            }
            if typing.backspace {
                if self.since_backspace > BACKSPACE_DELAY {
                    self.code_changed = true;
                    if !self.backspace_pressed {
                        self.since_backspace = 0.0;
                        self.backspace_pressed = true;
                    } else {
                        self.since_backspace = BACKSPACE_DELAY - BACKSPACE_REPEAT_SPEED;
                    }
                    self.code.pop();
                }
            }
        }

        if self.code_changed {
            self.code_lines = get_code_lines(&self.code.as_str());
        }
    }

    pub fn set_draw_lines<T>(&mut self, font_manager: &'a FontManager<T>) -> Result<(), String>{
        if self.code_changed {
            self.line_draws.clear();
            for (i, l) in self.code_lines.iter().enumerate() {
                if l.len() != 0 {
                    self.line_draws.push(
                        font_manager.get_draw_at_vec2(
                            &self.mono_font,
                            l,
                            TEXT_HEIGHT,
                            Vec2::new(0.0, ((TEXT_HEIGHT + 2) as usize * i) as f64)
                            )?
                    );
                }
            }
        }

        Ok(())
    }

    pub fn get_draw_code(&self) -> impl Iterator<Item=&TextDraw> {
        self.line_draws.iter()
    }
}

fn get_code_lines(code : &str) -> Vec<String> {
    let mut lines : Vec<String> = Vec::new();
    for l in code.split('\n') {
        lines.push(l.to_string());
    }

    lines
}
