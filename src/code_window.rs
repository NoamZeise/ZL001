use crate::TextDraw;
use crate::input::Typing;
use crate::FontManager;
use crate::resource::Font;
use crate::geometry::Vec2;

use std::iter::Iterator;

const TEXT_HEIGHT : u32 = 25;
const BACKSPACE_DELAY : f64 = 0.6;
const BACKSPACE_REPEAT_SPEED : f64 = 0.05;
const CURSOR_BLINK_DELAY : f64 = 1.2;
const CURSOR_BLINK_DURATION : f64 = 0.6;


pub struct CodeWindow<'a> {
    code : String,
    code_index : usize,
    code_lines : Vec<String>,
    line_draws : Vec<TextDraw<'a>>,
    code_changed : bool,
    since_backspace : f64,
    backspace_pressed : bool,
    enter_pressed : bool,
    cursor_blink_timer : f64,
    cursor_blink_updated : bool,
    mono_font : Font,
    prev_input : Typing,
}


impl<'a> CodeWindow<'a> {
    pub fn new(mono_font : Font) -> Self {
        CodeWindow {
            code: String::new(),
            code_index: 0,
            code_lines: Vec::new(),
            line_draws: Vec::new(),
            since_backspace : BACKSPACE_DELAY,
            backspace_pressed : false,
            enter_pressed : false,
            cursor_blink_timer : 0.0,
            cursor_blink_updated : false,
            code_changed : false,
            mono_font,
            prev_input: Typing::new(),
        }
    }

    pub fn update(&mut self, frame_elapsed: f64, typing: &mut Typing) {
        self.code_changed = false;
        self.since_backspace += frame_elapsed;
        self.cursor_blink_timer += frame_elapsed;
        if self.cursor_blink_timer > CURSOR_BLINK_DELAY {
            self.cursor_blink_timer = 0.0;
            self.code_changed = true;
            self.cursor_blink_updated = false;
        } else if self.cursor_blink_timer > CURSOR_BLINK_DURATION && !self.cursor_blink_updated{
            self.cursor_blink_updated = true;
            self.code_changed = true;
        }
        if !typing.ctrl  && !typing.backspace && !typing.enter && !typing.tab {
            self.since_backspace = BACKSPACE_DELAY;
            self.backspace_pressed = false;
            self.enter_pressed = false;
            match typing.character {
                Some(c) => {
                    self.code_changed = true;
                    self.code.insert(self.code_index, c);
                    self.code_index+=1;
                },
                None => {
                        if typing.left && !self.prev_input.left {
                            if self.code_index != 0 {
                                self.code_index-=1;
                                self.code_changed = true;
                            }
                        }
                        if typing.right && !self.prev_input.right {
                            if self.code_index != self.code.len() {
                                self.code_changed = true;
                                self.code_index+=1;
                            }
                        }
                    if typing.up && !self.prev_input.up {
                        let line_offset = get_line_start_index(&self.code, self.code_index);
                        let last_line_offset = get_line_start_index(&self.code, line_offset);
                        let mut line_off = self.code_index - line_offset;
                        if last_line_offset == 0 && line_off != 0 {
                            line_off -= 1;
                        }
                        let last_line_len = line_offset - last_line_offset;
                        if last_line_len < line_off {
                            line_off = last_line_len;
                        }
                        self.code_index = last_line_offset + line_off;
                        self.code_changed = true;
                    }
                    if typing.down && !self.prev_input.down {
                        let line_start_i = get_line_start_index(&self.code, self.code_index);
                        let next_line_start_i = get_next_line_index(&self.code, self.code_index);
                        let second_line_start_i = get_next_line_index(&self.code, next_line_start_i + 1);
                        let mut line_off = self.code_index - line_start_i;
                        let next_line_length = second_line_start_i - next_line_start_i;
                        if next_line_length < line_off {
                            line_off = next_line_length;
                        }
                        if line_start_i == 0  && line_off != next_line_length { line_off += 1;}
                        if self.code.len() != 0 && next_line_start_i >= self.code.len() - 1 {
                            line_off = 1;
                        }
                        self.code_index = next_line_start_i + line_off;
                        if self.code_index > self.code.len() { self.code_index = self.code.len(); }
                        self.code_changed = true;
                    }
                    
                },
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
                self.code.insert(self.code_index, '\n');
                self.code_index+=1;
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
                    if self.code_index != 0 {
                        self.code.remove(self.code_index - 1);
                        self.code_index-=1;
                    }
                }
            } else if typing.tab && !self.prev_input.tab {
                self.code.insert_str(self.code_index, &"    ");
                self.code_index+=4;
                self.code_changed = true;
            }
        }

        if self.code_changed {
            let mut cursor = '█';
            if self.cursor_blink_timer < CURSOR_BLINK_DURATION {
                cursor = ' ';
            }
            self.code_lines = get_code_lines(&self.code.as_str(), self.code_index, cursor);
        }
        self.prev_input = *typing;
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

    pub fn get_code(&self) -> &str {
        &self.code
    }
}

fn get_code_lines(code : &str, cursor_index : usize, cursor : char) -> Vec<String> {
    let mut lines : Vec<String> = Vec::new();
    let mut index = 0;
    let mut prev_index = 0;
    let mut placed_cursor = false;
    for l in code.split('\n') {
        index += l.len() + 1;
        let mut l = l.to_string();
        if index > cursor_index && !placed_cursor {
            l.insert(cursor_index - prev_index, cursor);
            placed_cursor = true;
        }
        lines.push(l);
        prev_index = index;    
    }

    lines
}

fn get_line_start_index(code : &str, index : usize) -> usize {
    let mut index = index;
    while index > 0 {
        index -= 1;
        if code.chars().nth(index).unwrap() == '\n' {
            break;
        }
    }
    index
}

fn get_next_line_index(code : &str, index : usize) -> usize {
    if code.len() == 0 { return 0; }
    let mut index = index;
    while index <= code.len() - 1 {
        if code.chars().nth(index).unwrap() == '\n' {
            break;
        }
        index += 1;
    }
    index
}
