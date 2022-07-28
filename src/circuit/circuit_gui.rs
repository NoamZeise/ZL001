use crate::{
    GameObject, geometry::*, input::Mouse, TextureManager, FontManager, resource::Font, microcontroller::Microcontroller};
use super::button::Button;
use super::circuit_helper::McConnection;
use sdl2::video::Window;
use sdl2::render::Canvas;

use std::collections::HashMap;

const CONNECTION_THICKNESS : f64 = 10.0;
const CONNECTION_BTN_SIZE : f64 = 20.0;

#[derive(PartialEq)]
enum State {
    Default,
    AddMc,
    AddCon,
    McMenu,
}

pub struct Gui  {
    add_mc_btn : Button,
    add_con_btn : Button,
    clear_btn : Button,
    save_btn : Button,
    load_btn : Button,
    compile_btn : Button,
    step_btn : Button,
    remove_mc_btn : Button,
    code_mc_btn : Button,
    prev_mouse : Mouse,
    mc_btns : Vec<Button>,
    mc_cons : Vec<GameObject>,
    state : State,
    placed_rect : Option<Rect>,
    connection : Option<(McConnection, McConnection)>,
    clicked_connection : Option<McConnection>,
    con_btns : Vec<(Button, McConnection)>,
    prev_click_pos : Option<Vec2>,
    current_mouse_pos : Vec2,
    mc_selected_index : Option<usize>, 
    box_tex : GameObject,
    font : Font,
}

impl Gui {
    pub fn new(btn_obj : GameObject, font : Font) -> Self {
        let add_mc_btn = Button::new(btn_obj.clone(), Some(Rect::new(5.0, 3.0, 130.0, 26.0)), "add circuit".to_string());
        let add_con_btn = Button::new(btn_obj.clone(), Some(Rect::new(140.0, 3.0, 100.0, 26.0)), "add conn".to_string());
        
        let save_btn = Button::new(btn_obj.clone(), Some(Rect::new(560.0, 10.0, 75.0, 25.0)), "save".to_string());
        let load_btn = Button::new(btn_obj.clone(), Some(Rect::new(560.0, 40.0, 75.0, 25.0)), "load".to_string());
        let clear_btn = Button::new(btn_obj.clone(), Some(Rect::new(560.0, 70.0, 75.0, 25.0)), "clear".to_string());

        let compile_btn = Button::new(btn_obj.clone(), Some(Rect::new(560.0, 415.0, 75.0, 25.0)), "compile".to_string());
        let step_btn = Button::new(btn_obj.clone(), Some(Rect::new(560.0, 445.0, 75.0, 25.0)), "step".to_string());
        
        let remove_mc_btn = Button::new(btn_obj.clone(), Some(Rect::new(100.0, 400.0, 60.0, 30.0)), "del".to_string());
        let code_mc_btn = Button::new(btn_obj.clone(), Some(Rect::new(180.0, 400.0, 60.0, 30.0)), "code".to_string());
        
        Gui {
            add_mc_btn,
            add_con_btn,
            clear_btn,
            save_btn,
            load_btn,
            compile_btn,
            step_btn,
            remove_mc_btn,
            code_mc_btn,
            mc_btns : Vec::new(),
            mc_cons : Vec::new(), 
            prev_mouse : Mouse::new(),
            state : State::Default,
            placed_rect : None,
            connection : None,
            clicked_connection : None,
            con_btns : Vec::new(),
            prev_click_pos : None,
            current_mouse_pos : Vec2::new(0.0, 0.0),
            mc_selected_index : None,
            box_tex : btn_obj,
            font,
        }
    }
    
    pub fn draw<'sdl2, TTex, TFont>(&mut self, canvas : &mut Canvas<Window>,  texture_manager : &'sdl2 TextureManager<TTex>, font_manager : &'sdl2 FontManager<TFont>) -> Result<(), String> {
        self.add_mc_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
        self.add_con_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
        self.clear_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
        self.save_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
        self.load_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
        self.compile_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
        self.step_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
        for mc in self.mc_btns.as_slice() {
            mc.draw(canvas, texture_manager, font_manager, &self.font)?;
        }
        for con in self.mc_cons.as_slice() {
            texture_manager.draw(canvas, con)?;
        }
        match self.state {
            State::AddMc => {
                if let Some(p) = self.prev_click_pos {
                    self.box_tex.draw_rect = Rect::new_from_vec2s(&p, &self.current_mouse_pos);
                    texture_manager.draw(canvas, &self.box_tex)?;
                }
                texture_manager.draw_rect(canvas, &self.add_mc_btn.game_obj().draw_rect, &Rect::new(30.0, 60.0, 90.0, 100.0))?;
            },

            State::McMenu => {
                self.remove_mc_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
                self.code_mc_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
            },

            State::AddCon => {
                if let Some(p) = self.clicked_connection {
                    let lines = self.add_line(self.get_io_out_pos(&p), self.current_mouse_pos);
                    for l in lines {
                        texture_manager.draw(canvas, &l)?;
                    }
                }
                for c in self.con_btns.iter() {
                    c.0.draw(canvas, texture_manager, font_manager, &self.font)?;
                }
                texture_manager.draw_rect(canvas, &self.add_con_btn.game_obj().draw_rect, &Rect::new(30.0, 60.0, 90.0, 100.0))?;
            },
            _ => (),
        } 
        Ok(())
    }

    pub fn update(&mut self, mouse : &Mouse, mcs : &Vec<Microcontroller>, connections : &HashMap<McConnection, McConnection>, modified : bool) {
        if self.state == State::Default {
            self.mc_selected_index = None;
        }
        self.current_mouse_pos = Vec2::new(mouse.x as f64, mouse.y as f64);
        self.placed_rect = None;
        self.connection = None;

        if modified {
            self.mc_btns.clear();
            self.con_btns.clear();
            for (i, mc) in mcs.iter().enumerate() {
                self.mc_btns.push(Button::new(mc.get_game_object().clone(), None, "".to_string()));
                for j in 0..mc.io_count() {
                    let con = McConnection::new(i, j);
                    let p = self.get_io_out_pos(&con);
                    let rect = Rect::new(p.x - CONNECTION_BTN_SIZE/2.0, p.y - CONNECTION_BTN_SIZE/2.0, CONNECTION_BTN_SIZE, CONNECTION_BTN_SIZE);
                    let mut go = mc.get_game_object().clone();
                    go.draw_rect = rect;
                    self.con_btns.push(
                        (
                            Button::new(go, None, "+".to_string()),
                            con
                        )
                    );
                }
            }
            self.mc_cons.clear();
            for c in connections {
                self.add_connection(c);
            }
        }

        self.btn_update(mouse);
            
        if self.add_mc_btn.clicked() {
            if self.state == State::Default {
                self.state = State::AddMc;
            } else if self.state == State::AddMc {
                self.state = State::Default;
            }
        } else if self.state == State::AddMc {
            self.circ_place_mode_update(mouse);
        } else if self.state == State::McMenu {
            //TODO
            if !self.prev_mouse.left_click && mouse.left_click {
                self.state = State::Default;
            }
            
        } else if self.add_con_btn.clicked() {
            if self.state == State::Default {
                self.state = State::AddCon;
            } else if self.state == State::AddCon {
                self.state = State::Default;
            }
        } else if self.state == State::AddCon {
            self.con_place_mode_update(mouse);
        } else{
            self.prev_click_pos = None;
        }
        
        self.prev_mouse = *mouse;
    }

    fn add_connection(&mut self, con : (&McConnection, &McConnection)) {
        //temp way to add connection
        let p1 = self.get_io_out_pos(con.0);
        let p2 = self.get_io_out_pos(con.1);
        let lines = self.add_line(p1, p2);
        for l in lines {
            self.mc_cons.push(l);
        }
    }

    fn get_io_out_pos(&self, con : &McConnection) -> Vec2 {
        let mc_rect = self.mc_btns[con.get_mc_i()].game_obj().draw_rect.clone();
        let off = match con.get_io_i() {
            0 => Vec2::new(0.0, -mc_rect.h/2.0),
            1 => Vec2::new(mc_rect.w/2.0, 0.0),
            2 => Vec2::new(0.0, mc_rect.h/2.0),
            3 => Vec2::new(-mc_rect.w/2.0, 0.0),
            _ => panic!("io more than 3!"),
        };
        mc_rect.centre() + off
    }

    fn add_line(&mut self, p1: Vec2, p2: Vec2) -> Vec<GameObject> {
        let mut lines : Vec<GameObject> = Vec::new();
        let centre_x = (p1.x + p2.x) / 2.0;
        //p1 to centre
        lines.push(self.add_horizontal_line(p1, Vec2::new(centre_x, p1.y)));
        lines.push(self.add_horizontal_line(p2, Vec2::new(centre_x, p2.y)));
        lines.push(self.add_vertical_line(Vec2::new(centre_x, p1.y), Vec2::new(centre_x, p2.y)));
        lines
    }

    fn add_horizontal_line(&mut self, p1 :Vec2, p2: Vec2) -> GameObject {
        let (primary, secondary) = if p1.x < p2.x {
            (p1, p2)
        } else {
            (p2, p1)
        };
        let mut game_obj = self.box_tex.clone();
        game_obj.draw_rect = Rect::new(primary.x, primary.y, secondary.x - primary.x, CONNECTION_THICKNESS);
        game_obj
    }

    fn add_vertical_line(&mut self, p1 : Vec2, p2 : Vec2) -> GameObject {
        let (primary, secondary) = if p1.y < p2.y {
            (p1, p2)
        } else {
            (p2, p1)
        };
        let mut game_obj = self.box_tex.clone();
        game_obj.draw_rect = Rect::new(primary.x, primary.y, CONNECTION_THICKNESS, secondary.y - primary.y);
        game_obj
    }

    fn btn_update(&mut self, mouse : &Mouse) {
        if self.state == State::Default { 
            for (i, mc) in self.mc_btns.iter_mut().enumerate() {
                mc.update(mouse, &self.prev_mouse);
                if mc.clicked() {
                    self.prev_mouse.left_click = true;
                    self.mc_selected_index = Some(i);
                    self.state = State::McMenu;
                    //change pos of btn to be under circuit
                }
            }
        } else if self.state ==  State::McMenu {
            self.remove_mc_btn.update(mouse, &self.prev_mouse);
            self.code_mc_btn.update(mouse, &self.prev_mouse)
        }

        self.add_mc_btn.update(mouse, &self.prev_mouse);
        self.add_con_btn.update(mouse, &self.prev_mouse);
        self.clear_btn.update(mouse, &self.prev_mouse);
        self.save_btn.update(mouse, &self.prev_mouse);
        self.load_btn.update(mouse, &self.prev_mouse);
        self.compile_btn.update(mouse, &self.prev_mouse);
        self.step_btn.update(mouse, &self.prev_mouse);
    }

    fn circ_place_mode_update(&mut self, mouse : &Mouse) {
        if !self.prev_mouse.left_click && mouse.left_click { //clicking begin
            self.prev_click_pos = Some(self.current_mouse_pos);
            
        } else if self.prev_mouse.left_click && !mouse.left_click { //clicking end
            match self.prev_click_pos {
                Some(p) => {
                    self.placed_rect = Some(Rect::new_from_vec2s(&p, &self.current_mouse_pos));
                    if self.placed_rect.as_ref().unwrap().w < 30.0 ||
                        self.placed_rect.as_ref().unwrap().h < 30.0 {
                            self.placed_rect = None;
                        }
                    self.state = State::Default;
                },
                _ => (), // happens after cliicking add circuit
            }
        }
    }

    fn con_place_mode_update(&mut self, mouse : &Mouse) {
        for cb in self.con_btns.iter_mut() {
            cb.0.update(mouse, &self.prev_mouse);
        }
        if !self.prev_mouse.left_click && mouse.left_click { //clicking begin
            for cb in self.con_btns.iter() {
                if cb.0.selected() {
                    self.clicked_connection = Some(cb.1);
                    break;
                }
            }
            if self.clicked_connection == None {
                self.state = State::Default;
            }
        } else if self.prev_mouse.left_click && !mouse.left_click { //clicking end
            match self.clicked_connection {
                Some(c) => {
                    for cb in self.con_btns.iter() {
                        if cb.0.selected() {
                            if c.get_mc_i() != cb.1.get_mc_i() {
                                self.connection = Some((c, cb.1));
                            }
                            self.state = State::Default;
                            self.clicked_connection = None;
                            break;
                        }
                    }
                },
                _ => (), // happens after cliicking add circuit
            }
        }
    }

    pub fn add_circ_request(&self) -> Option<Rect> {
        self.placed_rect.clone()
    }

    pub fn add_con_request(&self) -> Option<(McConnection, McConnection)> {
        self.connection.clone()
    }

    pub fn remove_mcs_index(&mut self) -> Option<usize> {
        if self.remove_mc_btn.clicked() {
            let index = self.mc_selected_index;
            self.mc_selected_index = None;
            self.state = State::Default;
            self.remove_mc_btn.reset();
            return index;
        }

        None
    }

     pub fn code_mcs_index(&mut self) -> Option<usize> {
        if self.code_mc_btn.clicked() {
            let index = self.mc_selected_index;
            self.mc_selected_index = None;
            self.state = State::Default;
            self.code_mc_btn.reset();
            return index;
        }

        None
    }

    pub fn clear_circuit(&self) -> bool {
        self.clear_btn.clicked()
    }

    pub fn save_circuit(&self) -> bool {
        self.save_btn.clicked()
    }

    pub fn load_circuit(&self) -> bool {
        self.load_btn.clicked()
    }

    pub fn compile(&self) -> bool {
        self.compile_btn.clicked()
    }

    pub fn step(&self) -> bool {
        self.step_btn.clicked()
    }
}
