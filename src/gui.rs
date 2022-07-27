use crate::{button::Button, GameObject, geometry::*, input::Mouse, TextureManager, FontManager, resource::Font, microcontroller::Microcontroller};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

#[derive(PartialEq)]
enum State {
    Default,
    AddMc,
    McMenu,
}

pub struct Gui  {
    add_mc_btn : Button,
    clear_btn : Button,
    save_btn : Button,
    load_btn : Button,
    prev_mouse : Mouse,
    mc_btns : Vec<Button>,
    state : State,
    placed_rect : Option<Rect>,
    prev_click_pos : Option<Vec2>,
    current_mouse_pos : Vec2,
    mc_selected_index : Option<usize>, 
    box_tex : GameObject,
    font : Font,
}

impl Gui {
    pub fn new(btn_obj : GameObject, font : Font) -> Self {
        let add_mc_btn = Button::new(btn_obj.clone(), Some(Rect::new(5.0, 20.0, 150.0, 30.0)), "add circuit".to_string());
        let clear_btn = Button::new(btn_obj.clone(), Some(Rect::new(170.0, 20.0, 100.0, 30.0)), "clear".to_string());
        let save_btn = Button::new(btn_obj.clone(), Some(Rect::new(280.0, 20.0, 100.0, 30.0)), "save".to_string());
        let load_btn = Button::new(btn_obj.clone(), Some(Rect::new(390.0, 20.0, 100.0, 30.0)), "load".to_string());
        Gui {
            add_mc_btn,
            clear_btn,
            save_btn,
            load_btn,
            mc_btns : Vec::new(),
            prev_mouse : Mouse::new(),
            state : State::Default,
            placed_rect : None,
            prev_click_pos : None,
            current_mouse_pos : Vec2::new(0.0, 0.0),
            mc_selected_index : None,
            box_tex : btn_obj,
            font,
        }
    }

    fn draw_button<'sdl2, TTex, TFont>(
        canvas : &mut Canvas<Window>,
        texture_manager : &'sdl2 TextureManager<TTex>,
        font_manager : &'sdl2 FontManager<TFont>,
        font : &Font, button : &Button) -> Result<(), String> {
        texture_manager.draw(canvas, button.game_obj())?;
        if button.has_text() {
            let draw = font_manager.get_draw_at_vec2(
                font,
                button.text(),
                (button.game_obj().draw_rect.h * 0.9) as u32,
                Vec2::new(
                    button.game_obj().draw_rect.x + button.game_obj().draw_rect.w * 0.03,
                    button.game_obj().draw_rect.y,
                ),
                Color::RGB(140, 80, 20)
            )?;
            canvas.copy(&draw.tex, None, draw.rect)?;
        }
        if button.selected() {
            texture_manager.draw_rect(canvas, &button.game_obj().draw_rect, &Rect::new(40.0, 40.0, 40.0, 80.0))?;
        }
        Ok(())
    }
    
    pub fn draw<'sdl2, TTex, TFont>(&mut self, canvas : &mut Canvas<Window>,  texture_manager : &'sdl2 TextureManager<TTex>, font_manager : &'sdl2 FontManager<TFont>) -> Result<(), String> {
        Self::draw_button(canvas, texture_manager, font_manager, &self.font, &self.add_mc_btn)?;
        Self::draw_button(canvas, texture_manager, font_manager, &self.font, &self.clear_btn)?;
        Self::draw_button(canvas, texture_manager, font_manager, &self.font, &self.save_btn)?;
        Self::draw_button(canvas, texture_manager, font_manager, &self.font, &self.load_btn)?;
        for mc in self.mc_btns.as_slice() {
            Self::draw_button(canvas, texture_manager, font_manager, &self.font, mc)?;
        }
        if self.state == State::AddMc {
            if let Some(p) = self.prev_click_pos {
                self.box_tex.draw_rect = Rect::new_from_vec2s(&p, &self.current_mouse_pos);
                texture_manager.draw(canvas, &self.box_tex)?;
            }
            texture_manager.draw_rect(canvas, &self.add_mc_btn.game_obj().draw_rect, &Rect::new(30.0, 60.0, 90.0, 100.0))?;   
        }
        Ok(())
    }

    pub fn update(&mut self, mouse : &Mouse, mcs : &Vec<Microcontroller>, modified : bool) {
        self.current_mouse_pos = Vec2::new(mouse.x as f64, mouse.y as f64);
        self.placed_rect = None;
        self.mc_selected_index = None;

        if modified {
            self.mc_btns.clear();
            for mc in mcs {
                self.mc_btns.push(Button::new(mc.get_game_object().clone(), None, "".to_string()));
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
        } else if self.mc_selected_index.is_some() {
            //TODO
        }else {
            self.prev_click_pos = None;
        }
        
        self.prev_mouse = *mouse;
    }

    fn btn_update(&mut self, mouse : &Mouse) {
        if self.state == State::Default { 
            for (i, mc) in self.mc_btns.iter_mut().enumerate() {
                mc.update(mouse, &self.prev_mouse);
                if mc.clicked() {
                    self.mc_selected_index = Some(i);
                }
            }
        }
        if self.mc_selected_index.is_none() {
            self.add_mc_btn.update(mouse, &self.prev_mouse);
            self.clear_btn.update(mouse, &self.prev_mouse);
            self.save_btn.update(mouse, &self.prev_mouse);
            self.load_btn.update(mouse, &self.prev_mouse);
        }
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

    pub fn add_circ_request(&self) -> Option<Rect> {
        self.placed_rect.clone()
    }

    pub fn remove_mcs_index(&self) -> Option<usize> {
        self.mc_selected_index //temp for test
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
}
